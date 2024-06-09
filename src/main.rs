use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use rand::{Rng, thread_rng};
use actix_web_prometheus::{PrometheusMetrics, PrometheusMetricsBuilder};
use std::collections::HashMap;
use tracing::{info,warn,error};
use tracing_subscriber;
use tracing_actix_web::TracingLogger;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Hello world");
    HttpResponse::Ok().body("Hello world")
}

#[get("/health")]
async fn health() -> impl Responder {
    let num = thread_rng().gen_range(1..10);

    match num {
        1 => {
            info!("Status: Ok");
            HttpResponse::Ok().body("Status: Ok")
        },
        2 => {
            warn!("Status: BadRequest");
            HttpResponse::BadRequest().body("Status: BadRequest")
        },
        3 => {
            warn!("Status: Too Many Request");
            HttpResponse::TooManyRequests().body("Status: Too Many Requests")
        },
        4 => {
            error!("Status: NotFound");
            HttpResponse::NotFound().body("Status: NotFound")
        },
        _ => {
            error!("Status: InternalServerError");
            HttpResponse::InternalServerError().body("Status: InternalServerError")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber 
    tracing_subscriber::fmt()
        .init();


    
    // Initialize Prometheus 
    let mut labels = HashMap::new();
    labels.insert("label".to_string(), "value1".to_string());

    let _prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build() 
        .unwrap(); 


    // Web Server 
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(_prometheus.clone())
            .service(hello)
            .service(health)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    
}
