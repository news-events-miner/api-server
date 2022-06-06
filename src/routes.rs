use crate::k8s::*;
use crate::types::*;
use actix_web::{get, post, web, Responder};
// use apiexts::CustomResourceDefinition;
use awc;
// use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts;
use kube::api::{Api, Patch, PatchParams};
use kube::Client;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

/// Check Spark job status
#[get("/job")]
async fn get_job(_job: web::Json<Job>) -> impl Responder {
    let existent_job = NewJob {
        name: String::from("SomeJob")
    };
    web::Json(existent_job)
}

/// Add new Spark job
#[post("/job")]
async fn add_job(job_params: web::Json<NewJob>) -> impl Responder {
    let client = Client::try_default().await.unwrap();
    let ssaply = PatchParams::apply("crd_spark").force();

    let apps: Api<SparkApplication> = Api::namespaced(client.clone(), "spark-operator");

    let mut env = Vec::new();

    for var in [
        "S3_PATH",
        "S3_ENDPOINT",
        "S3_ACCESS_KEY",
        "S3_SECRET_KEY",
        "DB_HOST",
        "DB_PORT",
        "DB_USER",
        "DB_PASS",
        "DB_NAME",
    ] {
        env.push(EnvVar {
            name: var.into(),
            value: env::var(var).unwrap(),
        })
    }

    let app_spec = SparkApplicationSpec {
        driver: Worker {
            coreLimit: "1200m".into(),
            cores: 1,
            env: env.clone(),
            labels: HashMap::from([("version".into(), "3.1.2".into())]),
            serviceAccount: "spark-k8s-spark".into(),
            memory: "1024M".into(),
        },
        executor: Worker {
            coreLimit: "1200m".into(),
            cores: 1,
            env,
            labels: HashMap::from([("version".into(), "3.1.2".into())]),
            serviceAccount: "spark-k8s-spark".into(),
            memory: "2048".into(),
        },
        image: "mkls0/event-extractor-spark:test".into(),
        imagePullPolicy: "Always".into(),
        mainApplicationFile: "file://opt/spark/python/custom_jobs/event_extractor/main.py".into(),
        mode: "cluster".into(),
        pythonVersion: "3".into(),
        sparkVersion: "3.1.2".into(),
        Type: "Python".into(),
    };

    // TODO: get from params
    let new_id = format!("{}-{}", job_params.name, Uuid::new_v4());
    let new_app = SparkApplication::new("spark-app", app_spec);

    println!("Applying new Spark App");

    let res = apps
        .patch("spark-app", &ssaply, &Patch::Apply(&new_app))
        .await
        .unwrap();

    web::Json(res)
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
