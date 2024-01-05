#![feature(lazy_cell)]

mod asset;
mod errors;
mod minify;
mod page;
mod routes;

use actix_web::{
    main as async_main,
    middleware,
    App,
    HttpServer,
};
use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The port to listen for connections on.
    #[arg(long, default_value = "8080")]
    port: u16,
    /// The log level to log stuff with.
    #[arg(long, default_value = "info")]
    log_level: log::LevelFilter,
}

#[async_main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    env_logger::builder()
        .filter_level(args.log_level)
        .target(env_logger::Target::Stdout)
        .format_timestamp(None)
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(errors::handler())
            .service(routes::handler())
    })
    .bind(("0.0.0.0", args.port))
    .with_context(|| format!("Failed to bind to 0.0.0.0:{}", args.port))?
    .run()
    .await
    .with_context(|| "Failed to run HttpServer")?;

    Ok(())
}
