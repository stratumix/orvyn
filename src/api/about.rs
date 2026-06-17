use actix_web::{HttpResponse, Responder, get};
use containerd_client::tonic::Request;
use serde::Serialize;

use containerd_client::services::v1::version_client::VersionClient;
use tracing::error;

use crate::instances::get_version_client;

#[derive(Serialize)]
struct VersionResponse {
    message: Option<String>,
    orvyn: String,
    containerd: String,
}

impl VersionResponse {
    fn new(orvyn: String, containerd: String) -> Self {
        Self {
            message: None,
            orvyn,
            containerd,
        }
    }
}

#[get("/")]
pub async fn version() -> impl Responder {
    let orvyn = env!("CARGO_PKG_VERSION").to_string();

    let mut client: VersionClient<_> = get_version_client().await;
    let request = Request::new(Default::default());

    match client.version(request).await {
        Ok(resp) => {
            let containerd = resp.into_inner().version;
            HttpResponse::Ok().json(VersionResponse::new(orvyn, containerd))
        }
        Err(err) => {
            error!(?err, "Failed to get containerd version");
            HttpResponse::InternalServerError()
                .json(VersionResponse::new(orvyn, "unknown".to_string()))
        }
    }
}
