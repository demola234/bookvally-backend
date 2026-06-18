use anyhow::Result;
use app::{
    config::AppConfig,
    consumers::spawn_consumers,
    container::Container,
    jobs::spawn_jobs,
    routes::all_routes,
    shutdown::shutdown_signal,
    telemetry::init_tracing,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_filename(".dev.env").ok();
    init_tracing();

    let config  = AppConfig::load()?;
    let timeout = config.server.shutdown_timeout_secs;
    let addr    = config.server.bind_addr();

    let container = Container::build(config).await?;

    spawn_consumers(container.clone());
    spawn_jobs(container.clone());

    let app      = all_routes(container);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on {addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(timeout))
        .await?;

    Ok(())
}
