use axum::{
    routing::get,
    Router,
};

mod about;
mod assets;
mod index;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/about", get(about::handler))
        .route("/assets/*path", get(assets::handler))
}
