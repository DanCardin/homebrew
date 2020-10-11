use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod batch;
pub mod beer;
pub mod measurement;
pub mod fermentable;

pub async fn check_health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
