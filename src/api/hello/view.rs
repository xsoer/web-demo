use axum::extract::{Path, Json};
use axum::http::status::StatusCode;

use super::schema;

pub async fn hello() -> &'static str {
    tracing::info!("hello");
    tracing::error!("errors is");
    "Hello, World!"
}


pub async fn a(Path(id): Path<u32>) -> (StatusCode, &'static str) {
    tracing::info!("id, {id}");
    (StatusCode::OK, "asd")
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<schema::CreateUser>,
) -> (StatusCode, Json<schema::User>) {
    tracing::info!("create user payload {payload:?}");
    // insert your application logic here
    let user = schema::User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

