use axum::Router;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

use crate::app::build_app_with_config;
use crate::config::GatewayRuntimeConfig;

pub async fn serve_standalone_gateway() -> Result<(), String> {
    let config = GatewayRuntimeConfig::from_env()?;
    let bind_address = config.bind_address.clone();
    let app = build_app_with_config(config).await?;
    serve_with_shutdown(app, &bind_address).await
}

async fn serve_with_shutdown(app: Router, bind_address: &str) -> Result<(), String> {
    let listener = TcpListener::bind(bind_address).await.map_err(|error| {
        format!("bind Skills standalone gateway on {bind_address} failed: {error}")
    })?;
    info!(bind_address, "sdkwork-skills standalone gateway listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|error| format!("serve Skills standalone gateway failed: {error}"))
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
