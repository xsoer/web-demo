use axum::extract::{Path, State, Json};
use axum::response::IntoResponse;

use crate::core::state::AppState;
use crate::util::{resp::{LibResult, Resp200}, error::LibError};
use super::schema;

pub async fn hello() -> LibResult<impl IntoResponse> {
    tracing::info!("hello");
    tracing::error!("errors is");
    Ok(Resp200::new("hello"))
}


pub async fn a(Path(id): Path<u64>) -> LibResult<impl IntoResponse> {
    tracing::info!("id, {id}");
    if id < 10 {
        return Err(
            LibError::ParamsErr(
                "user_id must be 42 digits and starts with 0x".to_string()
            )
        );
    }
    let result = schema::User {
        id,
        username: "test".to_string(),
    };
    Ok(Resp200::new(result))
}

pub async fn create_user(
    State(_app_state): State<AppState>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<schema::CreateUser>,
) -> LibResult<impl IntoResponse> {
    tracing::info!("create user payload {payload:?}");
    // insert your application logic here
    let user = schema::User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Resp200::new(user))
}

