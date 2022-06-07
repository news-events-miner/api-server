mod k8s;
mod routes;
mod types;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server");

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
