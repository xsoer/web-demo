use axum::Router;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

mod api;
mod core;
mod util;

#[tokio::main]
async fn main() {
    util::log::init();

    let app_state = core::state::init_state().await;
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = Router::new()
        .nest("/api", api::router())
        .layer(cors)
        .layer(core::auth::AuthLayer)
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let addr = get_addr();
    tracing::info!("server start");
    if let Err(e) = axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        tracing::error!("run err={e}")
    }
    tracing::info!("server stopped!")
}


fn get_addr() -> String {
    let run_host = dotenvy::var("RUN_HOST").expect(".env not found RUN_HOST");
    let run_port = dotenvy::var("RUN_PORT").expect(".env not found RUN_PORT");
    format!("{}:{}", run_host, run_port)
}