use axum::Router;
use sdkwork_api_skills_assembly::{
    assemble_app_surface_router, assemble_api_router,
    assemble_backend_surface_router,
};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

mod runtime;

use std::sync::Arc;

use crate::runtime::SkillsRuntime;

async fn build_app_router(runtime: Arc<SkillsRuntime>) -> Router {
    let service = runtime.service();
    let tenant_id = runtime.default_tenant_id();
    let pool = runtime.postgres_pool();
    assemble_app_surface_router(service, tenant_id, pool).await
}

async fn build_backend_router(runtime: Arc<SkillsRuntime>) -> Router {
    let service = runtime.service();
    let tenant_id = runtime.default_tenant_id();
    let pool = runtime.postgres_pool();
    assemble_backend_surface_router(service, tenant_id, pool).await
}

async fn serve_with_shutdown(app: Router, addr: &str, label: &str) -> Result<(), String> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|error| format!("bind {label} on {addr} failed: {error}"))?;
    info!("sdkwork-skills {label} listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|error| format!("serve {label} failed: {error}"))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

pub async fn serve_app_api(runtime: Arc<SkillsRuntime>) -> Result<(), String> {
    let addr = std::env::var("SDKWORK_SKILLS_APP_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18092".to_string());
    let app = build_app_router(runtime).await;
    serve_with_shutdown(app, addr.as_str(), "app api").await
}

pub async fn serve_backend_api(runtime: Arc<SkillsRuntime>) -> Result<(), String> {
    let addr = std::env::var("SDKWORK_SKILLS_BACKEND_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18093".to_string());
    let app = build_backend_router(runtime).await;
    serve_with_shutdown(app, addr.as_str(), "backend api").await
}

pub async fn serve_standalone_gateway(runtime: Arc<SkillsRuntime>) -> Result<(), String> {
    let addr = std::env::var("SDKWORK_SKILLS_APPLICATION_PUBLIC_INGRESS_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18092".to_string());
    let service = runtime.service();
    let tenant_id = runtime.default_tenant_id();
    let pool = runtime.postgres_pool();
    let app = assemble_api_router(service, tenant_id, pool)
        .await
        .router;
    serve_with_shutdown(app, addr.as_str(), "standalone gateway").await
}

pub async fn bootstrap_runtime() -> Result<Arc<SkillsRuntime>, String> {
    Ok(Arc::new(SkillsRuntime::bootstrap_from_env().await?))
}
