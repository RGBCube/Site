#![feature(iterator_try_collect, lazy_cell, let_chains)]

mod asset;
mod errors;
mod markdown;
mod minify;
mod page;
mod routes;

use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use actix_web::{
    main as async_main,
    middleware,
    App,
    HttpServer,
};
use anyhow::Context;
use clap::Parser;
use rustls::{
    pki_types::PrivateKeyDer,
    ServerConfig,
};
use rustls_pemfile::{
    certs,
    rsa_private_keys,
};

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

#[async_main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    env_logger::builder()
        .filter_level(args.log_level)
        .target(env_logger::Target::Stdout)
        .format_timestamp(None)
        .init();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(errors::handler())
            .service(routes::handler())
    });

    let server = if let Some(certificate_path) = args.certificate
        && let Some(key_path) = args.key
    {
        let certificates = certs(&mut BufReader::new(
            File::open(certificate_path).with_context(|| {
                format!(
                    "Failed to open certificate file at {}",
                    certificate_path.display()
                )
            })?,
        ));

        let mut keys = rsa_private_keys(&mut BufReader::new(
            File::open(key_path)
                .with_context(|| format!("Failed to open key file at {}", key_path.display()))?,
        ));

        let tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                certificates.try_collect::<Vec<_>>()?,
                PrivateKeyDer::Pkcs1(keys.next().unwrap()?),
            )
            .unwrap();

        server.bind_rustls(("0.0.0.0", args.port), tls_config)
    } else {
        server.bind(("0.0.0.0", args.port))
    };

    server
        .with_context(|| format!("Failed to bind to 0.0.0.0:{}", args.port))?
        .run()
        .await
        .with_context(|| "Failed to run HttpServer")?;

    Ok(())
}
