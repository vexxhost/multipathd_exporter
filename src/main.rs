mod collector;

use crate::collector::MultipathdCollector;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let collector = MultipathdCollector::new();
    prometheus::register(Box::new(collector)).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/metrics", get(metrics));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10035")
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "MultipathD Exporter\n".to_string())
}

async fn metrics() -> impl IntoResponse {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();

    (
        StatusCode::OK,
        encoder.encode_to_string(&metric_families).unwrap(),
    )
}
