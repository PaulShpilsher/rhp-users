use actix_web::{
    error, get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

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

async fn register(info: web::Json<User>) -> impl Responder {
    //HttpResponse::Ok().body("Welcome")
    // HttpResponse::Ok().body(format!("Welcome {}, {}, {}!", info.username, info.email, info.password))

    let result = UserResponse {
        id: 10,
        username: info.username.to_owned(),
        email: info.email.to_owned()
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
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            });

        App::new().service(hello).service(
            web::scope("/api")
                .app_data(json_config)
                .route("/register", web::post().to(register)),
        )
    })
    .bind(("127.0.0.1", 8180))?
    .run()
    .await
}
