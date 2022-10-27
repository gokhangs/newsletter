//! tests/health_check.rs
use actix_web::web::Data;
use reqwest;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use tokio;
use uuid::Uuid;
use zero2prod::{configuration, startup};
use sqlx::Executor;
pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

/// Spins up instance of the application and returns address
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration =
        configuration::get_configuration().expect("Failed to get configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&configuration.database).await;

    let server = startup::run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address, pool }
}

pub async fn configure_database(config: &configuration::DatabaseSettings) -> PgPool {
    // Create DB
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate DB
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    let app = spawn_app().await;
    //let configuration = configuration::get_configuration().expect("Failed to read configuration");
    //let connection_string = configuration.database.connection_string();

    let client = reqwest::Client::new();
    let body = "name=ket&email=ket%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ket@gmail.com");
    assert_eq!(saved.name, "ket");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //cases for the 'parameterised test'
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=gokhansa%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", app.address.clone()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request with Payload: {}",
            error_message
        );
    }
}
