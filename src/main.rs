mod k8s;
mod routes;
mod types;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use routes::*;
use std::env;
use std::process::exit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server");

    let vars = [
        "DB_HOST",
        "DB_PORT",
        "DB_USER",
        "DB_PASS",
        "DB_NAME",
        "S3_ENDPOINT",
        "S3_ACCESS_KEY",
        "S3_SECRET_KEY",
    ];

    for key in vars {
        let var = env::var(key);

        if var.is_err() {
            eprintln!("ERROR: environment variable {} not set", key);
            exit(1);
        }
    }

    let server = HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_header();
        App::new()
            .wrap(cors)
            .service(get_job)
            .service(add_job)
            .service(get_events)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    println!("Server has started successfully");
    server.await
}
