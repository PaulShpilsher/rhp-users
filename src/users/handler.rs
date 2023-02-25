use actix_web::{web, Responder, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i64,
}

pub async fn register(info: web::Json<User>) -> Result<impl Responder> {
    debug!(
        "registering user {}, {}, {}",
        info.username, info.email, info.password
    );

    let result = UserResponse { id: 10 };

    Ok(web::Json(result))
}
