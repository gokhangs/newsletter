use std::net::TcpListener;

//use env_logger::Env;
use actix_web;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{configuration, startup, telemetry};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to get configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
 
    //println!("{}", listener.local_addr().unwrap().port());
    let pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Couldn't connect to the DB");
    startup::run(listener, pool)?.await
}