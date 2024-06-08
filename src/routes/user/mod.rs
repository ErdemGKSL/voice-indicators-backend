use axum::Router;

use self::id::get_router as id_router;

mod id;

pub fn get_router() -> Router<()> {
  Router::new()
    .nest("/:id", id_router())
}