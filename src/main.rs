use std::net::TcpListener;

//use env_logger::Env;
use actix_web;
use sqlx::PgPool;
use zero2prod::{configuration, startup, telemetry};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    //'set_global_default` can be used by applications to specify what subscriber should be used to process spans.
    //set_global_default(subscriber).expect("Failed to set subscriber");

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