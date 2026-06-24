use sdkwork_skills_api_server::{bootstrap_runtime, serve_backend_api};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let runtime = bootstrap_runtime()
        .await
        .expect("bootstrap sdkwork-skills runtime");
    serve_backend_api(runtime)
        .await
        .expect("serve sdkwork-skills backend api");
}
