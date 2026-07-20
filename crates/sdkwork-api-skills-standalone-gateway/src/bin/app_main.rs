use sdkwork_api_skills_standalone_gateway::{bootstrap_runtime, serve_standalone_gateway};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let runtime = bootstrap_runtime()
        .await
        .expect("bootstrap sdkwork-skills runtime");
    serve_standalone_gateway(runtime)
        .await
        .expect("serve sdkwork-skills standalone gateway");
}
