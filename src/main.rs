#![feature(iterator_try_collect, lazy_cell, let_chains)]

mod asset;
mod errors;
mod markdown;
mod minify;
mod page;
mod routes;

use std::path::PathBuf;

use actix_web::{
    main as async_main,
    middleware,
    App,
    HttpServer,
};
use anyhow::Context;
use clap::Parser;
use openssl::ssl::{
    SslAcceptor,
    SslFiletype,
    SslMethod,
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
        let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();

        builder
            .set_private_key_file(key_path, SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(certificate_path)
            .unwrap();

        server.bind_openssl(("0.0.0.0", args.port), builder)
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
