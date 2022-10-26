use actix_web;
use zero2prod::run;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    run()?.await
}
