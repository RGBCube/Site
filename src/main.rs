#![feature(lazy_cell)]

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
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{
        uri::Scheme,
        StatusCode,
        Uri,
    },
    response::Redirect,
    BoxError,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use tower_http::trace::TraceLayer;

#[derive(Parser, Clone)]
#[command(author, version, about)]
struct Cli {
    /// The HTTP port to listen for connections on
    #[arg(long, default_value = "8080")]
    http_port: u16,
    /// The HTTPS port to listen for connections on
    #[arg(long, default_value = "8443")]
    https_port: u16,
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

async fn redirect_http(args: Cli) {
    let http_port = args.http_port.to_string();
    let https_port = args.https_port.to_string();

    let make_https = move |host: String, uri: Uri| -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&http_port, &https_port);
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    };

    let redirect = move |Host(host): Host, uri: Uri| {
        async move {
            match make_https(host, uri) {
                Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
                Err(error) => {
                    log::warn!("Failed to convert URI to HTTPS: {error}");
                    Err(StatusCode::BAD_REQUEST)
                },
            }
        }
    };

    let address = SocketAddr::from(([0, 0, 0, 0], args.http_port));

    axum_server::bind(address)
        .serve(redirect.into_make_service())
        .await
        .with_context(|| "Failed to run redirect server")
        .unwrap();
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

    if let (Some(certificate_path), Some(key_path)) = (&args.certificate, &args.key) {
        tokio::spawn(redirect_http(args.clone()));

        let config = RustlsConfig::from_pem_file(certificate_path, key_path)
            .await
            .with_context(|| "Failed to create TLS configuration from PEM files")?;

        let address = SocketAddr::from(([0, 0, 0, 0], args.https_port));

        axum_server::bind_rustls(address, config).serve(app).await
    } else {
        let address = SocketAddr::from(([0, 0, 0, 0], args.http_port));

        axum_server::bind(address).serve(app).await
    }
    .with_context(|| "Failed to run main server")?;

    Ok(())
}
