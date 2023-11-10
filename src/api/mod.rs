use axum::{Router, routing::{get, post}};
use crate::core::state::AppState;
pub mod hello;

pub fn router()-> Router<AppState>{
    Router::new()
        .route("/hello", get(hello::view::hello))
        .route("/hello/:id", get(hello::view::a))
        .route("/hello/", post(hello::view::create_user))
}