use std::sync::Arc;

use axum::Router;
use sdkwork_api_skills_assembly::{assemble_api_router_from_env, ApiAssembly};
use sdkwork_iam_web_adapter::{
    iam_web_request_context_resolver_from_env, IamAppContextInjector, IamAuthorizationPolicy,
};
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_bootstrap::{
    mount_infra_routes, mount_openapi_json, OpenApiMount, ServiceRouterConfig,
};
use sdkwork_web_core::{
    DefaultRateLimitPolicyResolver, HttpMetricsRegistry, WebRequestContextProfile,
    WebRequestContextResolver,
};

use crate::config::GatewayRuntimeConfig;

pub async fn build_app_from_env() -> Result<Router, String> {
    build_app_with_config(GatewayRuntimeConfig::from_env()?).await
}

pub async fn build_app_from_assembly(assembly: ApiAssembly) -> Result<Router, String> {
    let config = GatewayRuntimeConfig::from_env()?;
    let resolver = iam_web_request_context_resolver_from_env().await;
    build_app_with_resolver(assembly, resolver, config)
}

pub(crate) async fn build_app_with_config(config: GatewayRuntimeConfig) -> Result<Router, String> {
    let assembly = assemble_api_router_from_env().await?;
    let resolver = iam_web_request_context_resolver_from_env().await;
    build_app_with_resolver(assembly, resolver, config)
}

pub(crate) fn build_app_with_resolver<R>(
    assembly: ApiAssembly,
    resolver: R,
    config: GatewayRuntimeConfig,
) -> Result<Router, String>
where
    R: WebRequestContextResolver + Clone + Send + Sync + 'static,
{
    let public_path_prefixes = Vec::new();
    assembly
        .route_manifest
        .validate_public_path_prefixes(&public_path_prefixes)
        .map_err(|error| {
            format!("Skills route manifest public prefix validation failed: {error}")
        })?;

    let metrics = HttpMetricsRegistry::with_dimensions(config.metrics_dimensions);
    let authorization_policy =
        Arc::new(IamAuthorizationPolicy::new(assembly.route_manifest.clone()));
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(WebRequestContextProfile {
            public_path_prefixes,
            environment: config.environment,
            ..WebRequestContextProfile::default()
        })
        .with_security_policy(config.security_policy)
        .with_route_manifest(assembly.route_manifest)
        .with_authorization_policy(authorization_policy)
        .with_domain_injector(Arc::new(IamAppContextInjector))
        .with_rate_limit_resolver(Arc::new(DefaultRateLimitPolicyResolver))
        .with_metrics(metrics.clone());
    let layer = assembly
        .domain_context_injectors
        .into_iter()
        .fold(layer, |layer, injector| {
            layer.with_domain_injector(injector)
        });
    let business = with_web_request_context(assembly.router, layer);
    let business = mount_openapi_json(
        business,
        &[OpenApiMount {
            path: "/openapi.json",
            document: Arc::new(assembly.openapi),
        }],
    );

    Ok(mount_infra_routes(
        business,
        ServiceRouterConfig::default()
            .with_readiness_check(assembly.readiness_check)
            .with_metrics(metrics),
    ))
}

#[cfg(test)]
mod tests {
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use sdkwork_iam_web_adapter::IamWebRequestContextResolver;
    use sdkwork_web_bootstrap::{
        AlwaysReady, ReadinessCheck, ReadinessFuture, READINESS_DEPENDENCY_UNAVAILABLE,
    };
    use sdkwork_web_core::{DomainContextInjector, HttpRouteManifest, WebRequestContext};
    use tower::ServiceExt;

    use super::*;

    #[derive(Clone, Copy)]
    struct NoopDomainContextInjector;

    impl DomainContextInjector for NoopDomainContextInjector {
        fn inject(&self, _request: &mut axum::extract::Request, _context: &WebRequestContext) {}
    }

    #[derive(Clone, Copy)]
    struct FailingReadiness;

    impl ReadinessCheck for FailingReadiness {
        fn check(&self) -> ReadinessFuture<'_> {
            Box::pin(async {
                Err("postgres://skills:secret@database.internal/skills is unavailable".to_owned())
            })
        }
    }

    fn assembly(readiness_check: Arc<dyn ReadinessCheck>) -> ApiAssembly {
        ApiAssembly {
            owner: "sdkwork-skills",
            router: Router::new(),
            route_manifest: HttpRouteManifest::new(&[]),
            openapi: serde_json::json!({
                "openapi": "3.1.2",
                "info": { "title": "SDKWork Skills Test API", "version": "0.1.0" },
                "paths": {}
            }),
            permission_catalog: Vec::new(),
            readiness_check,
            domain_context_injectors: vec![Arc::new(NoopDomainContextInjector)],
        }
    }

    fn app(readiness_check: Arc<dyn ReadinessCheck>) -> Router {
        build_app_with_resolver(
            assembly(readiness_check),
            IamWebRequestContextResolver::new(None),
            GatewayRuntimeConfig::test(),
        )
        .expect("build Skills gateway")
    }

    async fn request(app: &Router, path: &str) -> axum::response::Response {
        app.clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("serve request")
    }

    async fn body_text(response: axum::response::Response) -> String {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read response body");
        String::from_utf8(bytes.to_vec()).expect("response body is UTF-8")
    }

    #[tokio::test]
    async fn standard_infrastructure_routes_are_mounted_once() {
        let app = app(Arc::new(AlwaysReady));
        for path in ["/healthz", "/livez", "/readyz", "/metrics", "/openapi.json"] {
            assert_eq!(request(&app, path).await.status(), StatusCode::OK, "{path}");
        }

        let openapi: serde_json::Value =
            serde_json::from_str(&body_text(request(&app, "/openapi.json").await).await)
                .expect("parse OpenAPI body");
        assert_eq!(openapi["info"]["title"], "SDKWork Skills Test API");

        let ready: serde_json::Value =
            serde_json::from_str(&body_text(request(&app, "/readyz").await).await)
                .expect("parse readiness body");
        assert_eq!(ready, serde_json::json!({ "status": "ready" }));

        let metrics = body_text(request(&app, "/metrics").await).await;
        for label in [
            "service=\"sdkwork-api-skills-standalone-gateway\"",
            "environment=\"test\"",
            "deployment_profile=\"standalone\"",
            "runtime_target=\"server\"",
            "runtime_profile=\"postgresql\"",
        ] {
            assert!(metrics.contains(label), "missing metrics label {label}");
        }
    }

    #[tokio::test]
    async fn readiness_failure_is_sanitized() {
        let response = request(&app(Arc::new(FailingReadiness)), "/readyz").await;
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = body_text(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).expect("parse readiness body");
        assert_eq!(json["status"], "not_ready");
        assert_eq!(json["detail"], READINESS_DEPENDENCY_UNAVAILABLE);
        for private_detail in ["database.internal", "skills:secret", "postgres://"] {
            assert!(!body.contains(private_detail));
        }
    }
}
