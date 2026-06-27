use tower_http::cors::CorsLayer;

pub mod aggregates;
pub mod config;
pub mod error;
pub mod extractors;
pub mod models;
pub mod queries;
pub mod repositories;
pub mod routes;
pub mod state;
pub mod views;

#[tokio::main]
async fn main() {
    let config = config::get();
    let state = state::get(&config).await;

    let app = routes::router().with_state(state);

    // CORS off in dev
    let app = if config.cors_should_be_permissive {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };

    // Serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
