mod internal_server_error;
mod not_found;

use actix_web::{
    http::StatusCode,
    middleware::ErrorHandlers,
};

pub fn handler<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found::handler)
        .handler(
            StatusCode::INTERNAL_SERVER_ERROR,
            internal_server_error::handler,
        )
}
