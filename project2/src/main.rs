/* An actix Microservice that has multiple routes:
A. / that returns a welcome page
B. /solve/n that returns the solution of the n-queens problem with board size n
C. /version that returns the version of the service
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use project2::n_queens;

extern crate log;
use log::{debug, error, log_enabled, info, Level};

// create a function that returns a welcome page
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("This is a n-queens solver microservice")
}

// create a function that returns the n-queens problem solution with board size n
#[get("/solve/{n}")]
async fn solve(n: web::Path<i32>) -> impl Responder {
    info!("providing n-queens problem solution with board size={n}");

    HttpResponse::Ok().body(n_queens(*n))
}

// create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    //add a print message to the console that the service is running
    info!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(solve).service(version))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
