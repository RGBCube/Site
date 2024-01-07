use axum::Router;

mod internal_server_error;
mod not_found;

pub fn handler<B: 'static>() -> ErrorHandlers<B> {
    Router::new().fallback(not_found::handler).handler(
        StatusCode::INTERNAL_SERVER_ERROR,
        internal_server_error::handler,
    )
}
