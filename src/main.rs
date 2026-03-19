mod cli;
mod client;
mod error;
mod exporter_metrics;
mod metrics;
mod scraper;

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    scraper: scraper::Scraper,
    metrics: Arc<RwLock<metrics::Metrics>>,
    exporter_metrics: Arc<RwLock<exporter_metrics::ExporterMetrics>>,
}

async fn scrape(State(state): State<AppState>) -> Response {
    let mut exporter_metrics = state.exporter_metrics.write().await;
    exporter_metrics.start_scrape_timer();

    let device = match state.scraper.get_device().await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("failed to get device: {}", e);
            exporter_metrics.observe_scrape(false);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("scrape failed: {}", e),
            )
                .into_response();
        }
    };

    let resources = match state.scraper.get_resource_usage().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("failed to get resource usage: {}", e);
            exporter_metrics.observe_scrape(false);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("scrape failed: {}", e),
            )
                .into_response();
        }
    };

    let mut metrics = state.metrics.write().await;
    metrics.update(&device, &resources);
    exporter_metrics.observe_scrape(true);
    exporter_metrics.update_allocator_stats();
    let output = metrics.gather();

    (axum::http::StatusCode::OK, output).into_response()
}

async fn exporter_metrics_handler(State(state): State<AppState>) -> Response {
    let exporter_metrics = state.exporter_metrics.read().await;
    let output = exporter_metrics.gather();
    (axum::http::StatusCode::OK, output).into_response()
}

async fn health() -> Response {
    (axum::http::StatusCode::OK, "OK").into_response()
}

async fn root() -> Response {
    (
        axum::http::StatusCode::OK,
        "Sagemcom Fast Prometheus Exporter",
    )
        .into_response()
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&args.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let host = if args.port == 80 {
        args.host.clone()
    } else {
        format!("{}:{}", args.host, args.port)
    };

    tracing::info!(
        host = %host,
        username = %args.username,
        bind = %args.bind,
        "starting sagemcom-fast-exporter"
    );

    let refresh_interval = std::time::Duration::from_secs(args.refresh_interval_secs());
    let auth_method = client::AuthMethod::from(args.auth_method.as_str());
    let client = client::Client::new(
        host,
        args.username.clone(),
        args.password.clone(),
        refresh_interval,
        auth_method,
    );
    let scraper = scraper::Scraper::new(client);

    let state = AppState {
        scraper,
        metrics: Arc::new(RwLock::new(metrics::Metrics::new())),
        exporter_metrics: Arc::new(RwLock::new(exporter_metrics::ExporterMetrics::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/scrape", get(scrape))
        .route("/metrics", get(exporter_metrics_handler))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.bind).await.unwrap();
    tracing::info!("listening on {}", args.bind);

    axum::serve(listener, app).await.unwrap();
}
