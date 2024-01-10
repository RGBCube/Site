#![feature(lazy_cell)]

mod asset;
mod errors;
mod markdown;
mod minify;
mod page;
mod routes;

use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use clap::Parser;
use tower_http::trace::TraceLayer;

#[derive(Parser, Clone)]
#[command(author, version, about)]
struct Cli {
    /// The port to listen for connections on
    #[arg(long, default_value = "4777")]
    port: u16,
    /// The log level to log stuff with
    #[arg(long, default_value = "info")]
    log_level: log::LevelFilter,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    env_logger::builder()
        .filter_level(args.log_level)
        .target(env_logger::Target::Stdout)
        .format_timestamp(None)
        .init();

    let address = SocketAddr::new("::".parse().unwrap(), args.port);

    let router = Router::new()
        .merge(routes::router())
        .merge(errors::router())
        .layer(TraceLayer::new_for_http());

    axum_server::bind(address)
        .serve(router.into_make_service())
        .await
        .with_context(|| "Failed to run server")
}
