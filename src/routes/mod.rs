use axum::{
    routing::get,
    Router,
};

mod assets;
mod index;
mod markdown;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/*page", get(markdown::handler))
        .route("/assets/*path", get(assets::handler))
}
