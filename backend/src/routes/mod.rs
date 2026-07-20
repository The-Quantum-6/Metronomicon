pub mod auth;
pub mod contribution;
pub mod course;
pub mod faq;
pub mod files;
pub mod link;
pub mod project_idea;
pub mod report;
pub mod resources;
use crate::state::AppState;
use axum::Router;

//pub fn router(state: AppState) -> Router<AppState> {
 //   Router::new()
       // .merge(user::router(state))

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(course::router())
        .merge(link::router())
        .merge(project_idea::router())
        .merge(faq::router())
        .merge(contribution::router())
        .merge(resources::router())
        .merge(report::router())
        .merge(auth::router())
        .merge(files::router())
}
