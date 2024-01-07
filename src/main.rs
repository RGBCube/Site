#![feature(lazy_cell, let_chains)]

mod asset;
mod errors;
mod markdown;
mod minify;
mod page;
mod routes;

use std::{
    net::SocketAddr,
    path::PathBuf,
};

use anyhow::Context;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use tower_http::trace::TraceLayer;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The port to listen for connections on
    #[arg(long, default_value = "8080")]
    port: u16,
    /// The log level to log stuff with
    #[arg(long, default_value = "info")]
    log_level: log::LevelFilter,

    /// The path to the certificate file
    #[arg(long)]
    certificate: Option<PathBuf>,
    /// The path to the key file
    #[arg(long)]
    key: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    env_logger::builder()
        .filter_level(args.log_level)
        .target(env_logger::Target::Stdout)
        .format_timestamp(None)
        .init();

    let app = Router::new()
        .merge(routes::router())
        .merge(errors::router())
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    let address = SocketAddr::from(([0, 0, 0, 0], args.port));

    if let Some(certificate_path) = args.certificate
        && let Some(key_path) = args.key
    {
        let config = RustlsConfig::from_pem_file(certificate_path, key_path)
            .await
            .with_context(|| "Failed to create TLS configuration from PEM files")?;

        axum_server::bind_rustls(address, config).serve(app).await
    } else {
        axum_server::bind(address).serve(app).await
    }
    .with_context(|| "Failed to run server")?;

    Ok(())
}
