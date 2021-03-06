use crate::k8s::*;
use crate::types::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use kube::api::{Api, Patch, PatchParams};
use kube::error::DiscoveryError::*;
use kube::error::Error as kube_error;
use kube::Client;
use std::collections::HashMap;
use std::env;

fn handle_kube_error(t: kube::error::Error) -> HttpResponse {
    match t {
        kube_error::Auth(err) => HttpResponse::Unauthorized().json(format!("{}", err)),
        kube_error::Api(api_err) => {
            let code = api_err.code;
            let reason = api_err.reason;
            let message = api_err.message;

            HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap())
                .json(HashMap::from([("reason", reason), ("message", message)]))
        }
        kube_error::Discovery(e) => match e {
            MissingKind(s) | MissingResource(s) | MissingApiGroup(s) => {
                HttpResponse::NotFound().json(HashMap::from([("reason", s)]))
            }
            _ => todo!(),
        },
        kube_error::SerdeError(e) => {
            HttpResponse::BadRequest().json(HashMap::from([("message", format!("{}", e))]))
        }
        _ => todo!(),
    }
}

/// Check Spark job status
#[get("/job")]
async fn get_job(job: web::Json<Job>) -> impl Responder {
    let client = Client::try_default().await.unwrap();
    let jobs: Api<SparkApplication> = Api::namespaced(client.clone(), "spark-operator");

    let job = jobs.get_status(job.name.as_str()).await;

    match job {
        Ok(job) => HttpResponse::Ok().json(job.status),
        Err(err) => handle_kube_error(err),
    }
}

/// Add new Spark job
#[post("/job")]
async fn add_job(job_params: web::Json<NewJob>) -> impl Responder {
    let client = Client::try_default().await.unwrap();
    let ssaply = PatchParams::apply("spark-operator").force();

    let apps: Api<SparkApplication> = Api::namespaced(client.clone(), "spark-operator");

    let mut env = Vec::new();

    for var in [
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

    env.push(EnvVar {
        name: "S3_PATH".into(),
        value: job_params.dataset_path.clone(),
    });

    let app_spec = SparkApplicationSpec {
        driver: Driver {
            core_limit: "1200m".into(),
            cores: job_params.driver_cores,
            env: env.clone(),
            labels: HashMap::from([("version".into(), "3.1.2".into())]),
            service_account: "spark-k8s-spark".into(),
            memory: "1024M".into(),
        },
        executor: Worker {
            core_limit: "1200m".into(),
            cores: job_params.worker_cores,
            instances: job_params.worker_instances,
            env,
            labels: HashMap::from([("version".into(), "3.1.2".into())]),
            service_account: "spark-k8s-spark".into(),
            memory: "2048M".into(),
        },
        image: "mkls0/event-extractor-spark:test".into(),
        image_pull_policy: "IfNotPresent".into(),
        main_app_file: "local:///opt/spark/python/custom_jobs/event_extractor/main.py".into(),
        mode: "cluster".into(),
        python_version: "3".into(),
        spark_version: "3.1.2".into(),
        app_type: "Python".into(),
    };

    let new_app = SparkApplication::new(job_params.name.as_str(), app_spec);

    println!("Applying new Spark App");

    let res = apps
        .patch(job_params.name.as_str(), &ssaply, &Patch::Apply(&new_app))
        .await;

    match res {
        Ok(_) => HttpResponse::Accepted().json("Application created successfully"),
        Err(err) => handle_kube_error(err),
    }
}

/// Get events within given time period
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

    // Possible missing values already handled in program entry
    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_user = env::var("DB_USER").unwrap();
    let db_pass = env::var("DB_PASS").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let url = format!("http://{}:{}/{}/_find", db_host, db_port, db_name);

    let client = awc::Client::default();
    let req = client
        .post(url)
        .basic_auth(db_user, db_pass)
        .content_type("application/json");
    let mut res = req.send_json(&selector).await.unwrap();

    let events = res.json::<Events>().limit(65535 * 128).await.unwrap();

    HttpResponse::Ok().json(events)
}
