use axum::{Router, routing::{get, post}};

mod get;
mod post;

pub fn get_router() -> Router<()> {
  Router::new()
    .route("/", get(get::trigger))
    .route("/", post(post::trigger))
}