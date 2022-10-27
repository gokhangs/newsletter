use std::net::TcpListener;

use actix_web;
use sqlx::PgPool;
use zero2prod::{configuration, startup};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to get configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    //let port = listener.local_addr().unwrap().port();
    println!("{}", listener.local_addr().unwrap().port());
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Couldn't connect to the DB");
    startup::run(listener, pool)?.await
}
