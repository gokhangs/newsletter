use std::net::TcpListener;

//use env_logger::Env;
use actix_web;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use zero2prod::{configuration, startup, telemetry};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to get configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    //println!("{}", listener.local_addr().unwrap().port());
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    startup::run(listener, pool)?.await
}
