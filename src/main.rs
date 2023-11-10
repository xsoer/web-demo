use axum::Router;

mod api;
mod core;
mod util;

#[tokio::main]
async fn main() {
    util::log::init();
    let app_state = core::state::init_state().await;
    let app = Router::new().nest("/api", api::router()).with_state(app_state);

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