use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use sdkwork_api_skills_assembly::{assemble_api_router_from_env, SkillsReadiness};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

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

async fn livez() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn readyz(
    State(readiness): State<SkillsReadiness>,
) -> Result<Json<Value>, (StatusCode, String)> {
    readiness
        .check()
        .await
        .map_err(|message| (StatusCode::SERVICE_UNAVAILABLE, message))?;
    Ok(Json(json!({ "status": "ok" })))
}

fn operations_router(readiness: SkillsReadiness) -> Router {
    Router::new()
        .route("/healthz", get(livez))
        .route("/livez", get(livez))
        .route("/readyz", get(readyz))
        .with_state(readiness)
}

pub async fn serve_standalone_gateway() -> Result<(), String> {
    let addr = std::env::var("SDKWORK_SKILLS_APPLICATION_PUBLIC_INGRESS_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18092".to_string());
    let assembly = assemble_api_router_from_env().await?;
    let app = operations_router(assembly.readiness).merge(assembly.router);
    serve_with_shutdown(app, addr.as_str(), "standalone gateway").await
}
