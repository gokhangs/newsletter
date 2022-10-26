use std::net::TcpListener;

use actix_web;
use zero2prod::run;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("{}", port);
    run(listener)?.await
}
