use std::net::TcpListener;

use actix_web::{HttpResponse, web};




#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(_form : web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}