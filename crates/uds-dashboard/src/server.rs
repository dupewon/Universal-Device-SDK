use std::net::SocketAddr;

#[cfg(feature = "full")]
pub struct DashboardServer {
    addr: SocketAddr,
    device_watch: Option<tokio::sync::watch::Receiver<Vec<uds_core::types::DeviceInfo>>>,
    log_ingester: Option<uds_logs::ingest::LogIngester>,
    telemetry: Option<uds_monitor::telemetry::TelemetryAggregator>,
    capture: Option<uds_monitor::capture::MonitorCapture>,
}

#[cfg(not(feature = "full"))]
pub struct DashboardServer;

impl DashboardServer {
    #[cfg(feature = "full")]
    pub fn new(
        addr: SocketAddr,
        device_watch: tokio::sync::watch::Receiver<Vec<uds_core::types::DeviceInfo>>,
        log_ingester: uds_logs::ingest::LogIngester,
        telemetry: uds_monitor::telemetry::TelemetryAggregator,
        capture: uds_monitor::capture::MonitorCapture,
    ) -> Self {
        Self { addr, device_watch: Some(device_watch), log_ingester: Some(log_ingester), telemetry: Some(telemetry), capture: Some(capture) }
    }

    #[cfg(not(feature = "full"))]
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "full")]
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        use axum::Router;
        use tower_http::cors::CorsLayer;
        use std::sync::Arc;

        let state = Arc::new(super::routes::AppState {
            device_watch: self.device_watch.clone().unwrap(),
            log_ingester: self.log_ingester.clone().unwrap(),
            telemetry: self.telemetry.clone().unwrap(),
            capture: self.capture.clone().unwrap(),
        });

        let app = Router::new()
            .route("/", axum::routing::get(super::routes::index))
            .route("/api/health", axum::routing::get(super::routes::health))
            .route("/api/devices", axum::routing::get(super::routes::list_devices))
            .route("/api/devices/{id}", axum::routing::get(super::routes::get_device))
            .route("/api/logs", axum::routing::get(super::routes::get_logs))
            .route("/api/metrics", axum::routing::get(super::routes::get_metrics))
            .route("/ws", axum::routing::get(super::ws::ws_handler))
            .layer(CorsLayer::permissive())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        tracing::info!("Dashboard server listening on http://{}", self.addr);
        axum::serve(listener, app).await?;
        Ok(())
    }

    #[cfg(not(feature = "full"))]
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Dashboard server requires the 'full' feature: cargo build --features full");
        Ok(())
    }
}
