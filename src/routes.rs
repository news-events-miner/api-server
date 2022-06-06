use crate::types::*;
use actix_web::{get, post, web, Responder};
use awc;
use std::collections::HashMap;
use std::env;

/// Check Spark job status
#[get("/job")]
async fn get_job(_job: web::Json<Job>) -> impl Responder {
    let existent_job = NewJob {
        name: String::from("SomeJob"),
        param1: String::from("value"),
    };
    web::Json(existent_job)
}

/// Add new Spark job
#[post("/job")]
async fn add_job(_job: web::Json<NewJob>) -> impl Responder {
    let new_job = Job {
        id: String::from("someCr4zyStuff"),
    };
    web::Json(new_job)
}

/// Get events by within given period
#[post("/events")]
async fn get_events(window: web::Json<TimeWindow>) -> impl Responder {
    let window_clone = window.clone();
    let hashmap = HashMap::from([
        (String::from("$gte"), window_clone.left),
        (String::from("$lte"), window_clone.right),
    ]);

    let selector = Selector {
        selector: DateFilter { date: hashmap },
    };

    // TODO: remove unwraps
    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_user = env::var("DB_USER").unwrap();
    let db_pass = env::var("DB_PASS").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let url = String::from(format!("http://{}:{}/{}/_find", db_host, db_port, db_name));

    let client = awc::Client::default();
    let req = client
        .post(url)
        .basic_auth(db_user, db_pass)
        .content_type("application/json");
    let mut res = req.send_json(&selector).await.unwrap();

    let events = res.json::<Events>().limit(65535 * 128).await.unwrap();

    web::Json(events)
}
