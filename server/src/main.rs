use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
use server::error::{ServerError, StackTrace};
use server::AppModule;
use server::routes;

#[tokio::main]
async fn main() -> Result<(), StackTrace> {
    let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "driver=debug,server=debug,tower_http=debug,hyper=debug,sqlx=debug".into())))
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG))
        .init();

    let _app = AppModule::new().await?;

    let router = Router::new()
        .route("/person", get(routes::person).post(routes::person_act))
        .with_state(_app);

    let bind = SocketAddr::from(([0, 0, 0 ,0], 8080));
    let tcpl = tokio::net::TcpListener::bind(bind).await
        .map_err(ServerError::from)?;

    axum::serve(tcpl, router.into_make_service())
        .await
        .map_err(ServerError::from)?;

    Ok(())
}