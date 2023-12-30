use actix_web::{
    main as async_main,
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
        .init();

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", args.port))
        .with_context(|| format!("Failed to bind to 0.0.0.0:{port}", port = args.port))?
        .run()
        .await
        .with_context(|| "Failed to run HttpServer")?;

    Ok(())
}
