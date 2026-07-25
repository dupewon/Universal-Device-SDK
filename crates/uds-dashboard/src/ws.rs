use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "full")]
pub async fn ws_handler(
    axum::extract::State(state): axum::extract::State<Arc<super::routes::AppState>>,
    ws: axum::extract::WebSocketUpgrade,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

#[cfg(feature = "full")]
async fn handle_socket(
    mut socket: axum::extract::ws::WebSocket,
    state: Arc<super::routes::AppState>,
) {
    use futures_util::SinkExt;
    use futures_util::StreamExt;

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut device_rx = state.device_watch.clone();

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let snapshot = state.telemetry.snapshot();
                let mut map = serde_json::Map::new();
                for m in &snapshot {
                    map.insert(m.name.clone(), serde_json::json!(m.value));
                }
                if let Err(e) = socket.send(axum::extract::ws::Message::Text(
                    serde_json::json!({
                        "type": "metric",
                        "metric": map
                    }).to_string()
                )).await {
                    tracing::debug!("WebSocket send error: {e}");
                    break;
                }
            }
            _ = device_rx.changed() => {
                let devices = device_rx.borrow().clone();
                if let Err(e) = socket.send(axum::extract::ws::Message::Text(
                    serde_json::json!({
                        "type": "device_update",
                        "devices": devices
                    }).to_string()
                )).await {
                    tracing::debug!("WebSocket send error: {e}");
                    break;
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(axum::extract::ws::Message::Close(_))) => break,
                    Some(Ok(axum::extract::ws::Message::Ping(data))) => {
                        if socket.send(axum::extract::ws::Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        tracing::debug!("WebSocket error: {e}");
                        break;
                    }
                    None => break,
                    _ => {}
                }
            }
        }
    }
}
