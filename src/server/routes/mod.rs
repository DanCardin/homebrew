use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod batch;
pub mod beer;
pub mod srm;

pub async fn check_health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
