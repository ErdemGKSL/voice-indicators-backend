use axum::Router;

use self::user::get_router as user_router;

mod user;

pub fn get_router() -> Router<()> {
  Router::new()
    .nest("/user", user_router())
}