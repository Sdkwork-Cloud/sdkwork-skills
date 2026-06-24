use std::sync::Arc;

use axum::Router;
use sdkwork_iam_web_adapter::IamDatabaseWebRequestContextResolver;
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_core::{
    DefaultRateLimitPolicyResolver, DomainContextInjector, WebRequestContext,
    WebRequestContextProfile,
};

use crate::http_route_manifest::app_route_manifest;
use crate::paths;
use crate::SkillsAppRequestContext;

pub fn skills_public_path_prefixes() -> Vec<String> {
    vec![
        paths::LIVEZ.to_owned(),
        paths::READYZ.to_owned(),
        paths::HEALTHZ.to_owned(),
    ]
}

#[derive(Clone, Default)]
struct SkillsAppContextInjector;

impl DomainContextInjector for SkillsAppContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        if let Some(app_context) = skills_app_context_from_web_request(context) {
            request.extensions_mut().insert(app_context);
        }
    }
}

fn skills_app_context_from_web_request(
    context: &WebRequestContext,
) -> Option<SkillsAppRequestContext> {
    let principal = context.principal.as_ref()?;
    let tenant_id = principal.tenant_id().parse().ok()?;
    let actor_id = principal.user_id().parse().ok();
    let organization_id = principal
        .organization_id()
        .and_then(|value| value.parse().ok());
    let session_id = principal.session_id().map(str::to_owned);
    Some(SkillsAppRequestContext {
        tenant_id,
        actor_id,
        organization_id,
        session_id,
    })
}

pub fn wrap_router_with_web_framework(
    resolver: IamDatabaseWebRequestContextResolver,
    router: Router,
) -> Router {
    let route_manifest = app_route_manifest();
    route_manifest
        .validate_public_path_prefixes(&skills_public_path_prefixes())
        .expect("skills app-api public prefixes must not cover protected manifest routes");

    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(WebRequestContextProfile {
            public_path_prefixes: skills_public_path_prefixes(),
            ..WebRequestContextProfile::default()
        })
        .with_route_manifest(route_manifest)
        .with_domain_injector(Arc::new(SkillsAppContextInjector))
        .with_rate_limit_resolver(Arc::new(DefaultRateLimitPolicyResolver));
    with_web_request_context(router, layer)
}

pub async fn wrap_router_with_web_framework_from_env(router: Router) -> Router {
    let resolver = sdkwork_iam_web_adapter::iam_database_resolver_from_env().await;
    wrap_router_with_web_framework(resolver, router)
}
