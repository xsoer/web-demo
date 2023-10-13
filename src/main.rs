use axum::Router;

mod api;
mod core;
mod util;

#[tokio::main]
async fn main() {
    util::log::init();

    let app = Router::new().nest("/api", api::router());

    // run our app with hyper, listening globally on port 3000
    let addr = format!("{}:{}", core::consts::HOST, core::consts::PORT);
    tracing::info!("server start");
    if let Err(e) = axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        tracing::error!("run err={e}")
    }
    tracing::info!("server stopped!")
}
