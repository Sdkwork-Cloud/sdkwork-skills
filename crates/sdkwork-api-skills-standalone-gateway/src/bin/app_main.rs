use sdkwork_api_skills_standalone_gateway::serve_standalone_gateway;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    sdkwork_database_sqlx::enable_process_shared_database_pool();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    serve_standalone_gateway()
        .await
        .expect("serve sdkwork-skills standalone gateway");
}
