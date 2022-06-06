mod routes;
mod types;

use actix_web::{App, HttpServer};
use routes::*;
use std::env;
use types::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_job)
            .service(add_job)
            .service(get_events)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
