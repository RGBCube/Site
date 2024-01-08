use axum::{
    // error_handling::HandleErrorLayer,
    Router,
};
// use tower::ServiceBuilder;

// mod internal_server_error;
mod not_found;

pub fn router() -> Router {
    Router::new().fallback(not_found::handler)
    // TODO
    // .layer(ServiceBuilder::new().
    // layer(HandleErrorLayer::new(internal_server_error::handler)))
}
