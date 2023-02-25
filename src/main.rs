use actix_web::{
    error, get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
    Responder,
};
use std::{env,option_env};
// use derive_more::{Display, Error};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse<'a> {
    id: i64,
    username: &'a String,
    email: &'a String,
}

async fn register(info: web::Json<User>) -> impl Responder {
    info!(
        "registering user {}, {}, {}",
        info.username, info.email, info.password
    );

    let result = UserResponse {
        id: 10,
        username: &info.username,
        email: &info.email,
    };

    let body = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
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

    let port = option_env!("PORT")
        .unwrap_or("8180")
        .parse::<u16>()
        .unwrap();
    info!("using port {}", port);

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
                .app_data(json_config)
                .route("/register", web::post().to(register)),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
