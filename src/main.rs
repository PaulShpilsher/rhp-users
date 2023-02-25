use actix_web::{
    error, get, guard,middleware::Logger, web, App, HttpResponse,
    HttpServer, Responder
};
use std::env;
use log::info;

mod config;
use config::Configuration;

mod users;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();


    let cfg = Configuration::new();
    
    info!("listenning on port {}", cfg.port);
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            });

        let logger = Logger::default();

        App::new().wrap(logger).service(hello).service(
            web::scope("/api")
                .guard(guard::Header("content-type", "application/json"))
                .app_data(json_config)
                .route("/register", web::post().to(users::register)),
        )
    })
    .bind(("127.0.0.1", cfg.port))?
    .run()
    .await
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
