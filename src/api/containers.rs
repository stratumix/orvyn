use actix_web::{HttpResponse, Responder, get, post, web};
use containerd_client::{
    services::v1::{Container, CreateContainerRequest, ListContainersRequest},
    tonic::{Request, metadata::MetadataValue},
};
use tracing::error;

use crate::instances::get_containers_client;

#[get("/containers")]
async fn list_containers() -> impl Responder {
    let mut client = get_containers_client().await;
    let mut request = Request::new(ListContainersRequest::default());
    request.metadata_mut().insert(
        "containerd-namespace",
        MetadataValue::from_static("default"),
    );

    match client.list(request).await {
        Ok(resp) => {
            let containers: Vec<String> = resp
                .into_inner()
                .containers
                .into_iter()
                .map(|c| c.id)
                .collect();

            HttpResponse::Ok().json(containers)
        }
        Err(err) => {
            error!(?err, "failed to list containers");
            HttpResponse::InternalServerError().body("failed")
        }
    }
}

#[post("/containers")]
async fn new_container(body: web::Json<Container>) -> impl Responder {
    let container = body.into_inner();

    let mut client = get_containers_client().await;
    let mut request = Request::new(CreateContainerRequest {
        container: Some(container),
    });
    request.metadata_mut().insert(
        "containerd-namespace",
        MetadataValue::from_static("default"),
    );

    match client.create(request).await {
        Ok(_resp) => HttpResponse::Created().finish(),
        Err(err) => {
            error!(?err, "Failed to create container");
            HttpResponse::InternalServerError().body("Failed to create container")
        }
    }
}
