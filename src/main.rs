mod api;
mod instances;

use tracing_subscriber::EnvFilter;

use crate::api::run_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    run_api().await;
}
