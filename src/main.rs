use actix_web::{
    error, get, guard, http::header::ContentType, middleware::Logger, web, App, HttpResponse,
    HttpServer, Responder, Result
};
use std::env;
// use derive_more::{Display, Error};
use log::info;
use serde::{Deserialize, Serialize};

mod config;
use config::Configuration;

#[derive(Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i64,
    username: String,
    email: String,
}

async fn register(info: web::Json<User>) ->  Result<impl Responder> {
    info!(
        "registering user {}, {}, {}",
        info.username, info.email, info.password
    );

    let result = UserResponse {
        id: 10,
        username: info.username.to_owned(),
        email: info.email.to_owned(),
    };

    Ok(web::Json(result))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
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
                .route("/register", web::post().to(register)),
        )
    })
    .bind(("127.0.0.1", cfg.port))?
    .run()
    .await
}
