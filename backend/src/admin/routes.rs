use axum::{routing::{delete, get, post, put}, Router};
use sqlx::PgPool;
use super::handlers;


//legge inn i main etterhvert: .nest("/admin", admin::routes::router())

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/tests", get(handlers::list_tests))
        .route("/tests", post(handlers::create_test))
        .route("/tests/{id}", get(handlers::get_test))
        .route("/tests/{id}", delete(handlers::delete_test))
        .route("/tests/{id}", put(handlers::update_test))
}