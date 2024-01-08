use axum::{
    routing::get,
    Router,
};

mod assets;
mod blog;
mod index;
mod markdown;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/blog", get(blog::index_handler))
        .route("/blog/:entry", get(blog::entry_handler))
        .route("/*path", get(assets::handler))
}
