pub(crate) mod api;
pub(crate) mod instances;

use crate::api::run_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    run_api().await;
}
