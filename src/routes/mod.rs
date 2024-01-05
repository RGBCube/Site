mod about;
mod assets;
mod index;

use actix_web::{
    web,
    Scope,
};

pub fn handler() -> Scope {
    web::scope("")
        .service(index::handler)
        .service(about::handler)
        .service(assets::handler)
}
