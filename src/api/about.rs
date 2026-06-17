use actix_web::{HttpResponse, Responder, get};

#[get("/")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}
