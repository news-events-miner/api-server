use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Job {
    id: String,
}

#[derive(Deserialize, Serialize)]
struct NewJob {
    name: String,
    param1: String,
}

#[get("/job")]
async fn get_job(_job: web::Json<Job>) -> impl Responder {
    let existent_job = NewJob {
        name: String::from("SomeJob"),
        param1: String::from("value"),
    };
    web::Json(existent_job)
}

#[post("/job")]
async fn add_job(_job: web::Json<NewJob>) -> impl Responder {
    let new_job = Job {
        id: String::from("someCr4zyStuff"),
    };
    web::Json(new_job)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_job).service(add_job))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
