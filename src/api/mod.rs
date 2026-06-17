use std::process::exit;

use actix_web::{App, HttpServer};
use tracing::{error, info};

use crate::api::{
    about::version,
    containers::{
        delete_container, get_container, list_containers, new_container, update_container,
    },
};

mod about;
mod containers;
mod structures;

pub async fn run_api() {
    let server = HttpServer::new(|| {
        App::new()
            .service(version)
            .service(list_containers)
            .service(get_container)
            .service(update_container)
            .service(new_container)
            .service(delete_container)
    })
    .bind(("127.0.0.1", 8080));

    match server {
        Ok(_) => info!("API webserver bound to 127.0.0.1:8080"),
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
