use super::routes;
use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
    middleware::Logger,
};
use sqlx::PgPool;
use std::net::TcpListener;
//web::Data wraps data in an ARC<T> pointer, which is always clonable regardless of <T>
use actix_web::web::Data;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
