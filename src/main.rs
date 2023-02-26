use actix_web::{
    error, get, guard, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;

mod config;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(None, NoTls).unwrap();

    // test db connection
    test_db_connection(pool.clone()).await;

    log::info!("listenning on port {}", cfg.port);
    HttpServer::new(move || {
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
                .app_data(web::Data::new(pool.clone()))
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

async fn test_db_connection(pool: Pool) {
    match pool.get().await {
        Ok(_) => log::info!("Connected to database"),
        Err(error) => panic!("Failed to connect to database {}", error)
    }
}