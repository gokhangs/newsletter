use actix_web::HttpResponse;
use once_cell::sync::Lazy;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
