use axum::{
    routing::get,
    Router,
};

mod assets;
mod index;
mod mdpage;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/:page", get(mdpage::handler))
        .route("/assets/*path", get(assets::handler))
}
