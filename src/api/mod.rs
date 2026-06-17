use std::process::exit;

use actix_web::{App, HttpServer};
use tracing::{error, info};

use crate::api::{
    about::version,
    containers::{list_containers, new_container},
};

mod about;
mod containers;

pub async fn run_api() {
    let server = HttpServer::new(|| {
        App::new()
            .service(version)
            .service(list_containers)
            .service(new_container)
    })
    .bind(("127.0.0.1", 8080));

    match server {
        Ok(_) => info!("API webserver binded to 127.0.0.1:8080"),
        Err(err) => {
            error!("Failed to bind API webserver: {err:?}");
            exit(1);
        }
    }
    if let Err(err) = server.unwrap().run().await {
        error!("Failed to start API webserver: {err:?}");
        exit(1);
    }
}
