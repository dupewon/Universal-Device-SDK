use std::sync::Arc;

pub struct AppState {
    pub device_watch: tokio::sync::watch::Receiver<Vec<uds_core::types::DeviceInfo>>,
    pub log_ingester: uds_logs::ingest::LogIngester,
    pub telemetry: uds_monitor::telemetry::TelemetryAggregator,
    pub capture: uds_monitor::capture::MonitorCapture,
}

#[cfg(feature = "full")]
pub async fn index() -> axum::response::Html<&'static str> {
    axum::response::Html(DASHBOARD_HTML)
}

#[cfg(feature = "full")]
pub async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[cfg(feature = "full")]
pub async fn list_devices(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> axum::Json<Vec<uds_core::types::DeviceInfo>> {
    axum::Json(state.device_watch.borrow().clone())
}

#[cfg(feature = "full")]
pub async fn get_device(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> axum::Json<Option<uds_core::types::DeviceInfo>> {
    let devices = state.device_watch.borrow();
    axum::Json(devices.iter().find(|d| d.id.0 == id).cloned())
}

#[cfg(feature = "full")]
pub async fn get_logs(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> axum::Json<Vec<serde_json::Value>> {
    use uds_logs::query::LogQuery;
    let results = state.log_ingester.query(LogQuery {
        level: None,
        pattern: None,
        limit: Some(100),
    });
    axum::Json(
        results
            .into_iter()
            .map(|e| {
                serde_json::json!({
                    "timestamp": e.timestamp,
                    "level": format!("{:?}", e.level),
                    "message": e.message,
                    "target": e.target
                })
            })
            .collect(),
    )
}

#[cfg(feature = "full")]
pub async fn get_metrics(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> axum::Json<serde_json::Value> {
    let snapshot = state.telemetry.snapshot();
    let mut map = serde_json::Map::new();
    for m in &snapshot {
        map.insert(m.name.clone(), serde_json::json!(m.value));
    }
    axum::Json(serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "metrics": map
    }))
}

#[cfg(not(feature = "full"))]
pub async fn health() -> &'static str {
    "Dashboard requires the 'full' feature"
}

#[cfg(not(feature = "full"))]
pub async fn list_devices() -> &'static str {
    "[]"
}

#[cfg(not(feature = "full"))]
pub async fn get_device() -> &'static str {
    "null"
}

#[cfg(not(feature = "full"))]
pub async fn get_logs() -> &'static str {
    "[]"
}

#[cfg(not(feature = "full"))]
pub async fn get_metrics() -> &'static str {
    "{}"
}

const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>UDS Dashboard</title>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #0f172a; color: #e2e8f0; }
header { background: #1e293b; padding: 1rem 2rem; border-bottom: 1px solid #334155; display: flex; align-items: center; gap: 1rem; }
header h1 { font-size: 1.25rem; color: #38bdf8; }
.container { max-width: 1400px; margin: 0 auto; padding: 2rem; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; }
.card { background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem; }
.card h2 { font-size: 1rem; color: #94a3b8; margin-bottom: 1rem; text-transform: uppercase; letter-spacing: 0.05em; }
.metric-value { font-size: 2rem; font-weight: 700; color: #38bdf8; }
.device-list { list-style: none; }
.device-list li { padding: 0.5rem 0; border-bottom: 1px solid #334155; display: flex; justify-content: space-between; }
.device-list li:last-child { border-bottom: none; }
.device-id { color: #e2e8f0; }
.device-status { color: #22c55e; }
.log-entry { font-family: "JetBrains Mono", "Fira Code", monospace; font-size: 0.8rem; padding: 0.25rem 0; color: #cbd5e1; }
.log-entry .warn { color: #f59e0b; }
.log-entry .error { color: #ef4444; }
.status-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; margin-right: 0.5rem; }
.status-online { background: #22c55e; }
.status-offline { background: #64748b; }
.status-error { background: #ef4444; }
</style>
</head>
<body>
<header>
    <h1>&#9670; UDS Dashboard</h1>
    <span id="status-badge" style="margin-left:auto;font-size:0.8rem;color:#94a3b8;">Connecting...</span>
</header>
<div class="container">
    <div class="grid">
        <div class="card">
            <h2>Devices</h2>
            <ul class="device-list" id="device-list">
                <li><span style="color:#64748b;">No devices found</span></li>
            </ul>
        </div>
        <div class="card">
            <h2>Metrics</h2>
            <div class="metric-value" id="metric-packets">0</div>
            <div style="color:#94a3b8;font-size:0.85rem;">Total Packets</div>
            <div class="metric-value" style="margin-top:1rem;" id="metric-devices">0</div>
            <div style="color:#94a3b8;font-size:0.85rem;">Connected Devices</div>
        </div>
        <div class="card">
            <h2>Health</h2>
            <div id="health-status">
                <span class="status-dot status-online"></span>
                <span id="health-text">Running</span>
            </div>
            <div style="margin-top:1rem;color:#94a3b8;font-size:0.85rem;">
                Uptime: <span id="uptime">0s</span>
            </div>
        </div>
    </div>
    <div class="card" style="margin-top:1.5rem;">
        <h2>Logs</h2>
        <div id="log-container" style="max-height:400px;overflow-y:auto;"></div>
    </div>
</div>
<script>
const startTime = Date.now();
const ws = new WebSocket(`${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}/ws`);
ws.onopen = () => document.getElementById('status-badge').textContent = 'Connected';
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (data.type === 'device_update') updateDeviceList(data.devices);
    if (data.type === 'log') appendLog(data.entry);
    if (data.type === 'metric') updateMetrics(data.metric);
};
ws.onclose = () => document.getElementById('status-badge').textContent = 'Disconnected';

async function fetchInitial() {
    try {
        const [devices, metrics] = await Promise.all([
            fetch('/api/devices').then(r => r.json()),
            fetch('/api/metrics').then(r => r.json()),
        ]);
        updateDeviceList(devices);
        if (metrics.metrics) {
            document.getElementById('metric-packets').textContent = metrics.metrics.packets || 0;
        }
        document.getElementById('metric-devices').textContent = devices.length;
    } catch (e) { console.error('Initial fetch failed', e); }
    setInterval(() => {
        const elapsed = Math.floor((Date.now() - startTime) / 1000);
        document.getElementById('uptime').textContent = `${elapsed}s`;
    }, 1000);
}

function updateDeviceList(devices) {
    document.getElementById('metric-devices').textContent = devices.length;
    const el = document.getElementById('device-list');
    if (devices.length === 0) { el.innerHTML = '<li><span style="color:#64748b;">No devices found</span></li>'; return; }
    el.innerHTML = devices.map(d => {
        const cls = d.connected ? 'status-online' : 'status-offline';
        return `<li><span class="device-id"><span class="status-dot ${cls}"></span>${d.id}</span><span class="device-status">${d.connected ? 'Online' : 'Offline'}</span></li>`;
    }).join('');
}

function appendLog(entry) {
    const el = document.getElementById('log-container');
    const cls = entry.level === 'Error' ? 'error' : entry.level === 'Warn' ? 'warn' : '';
    const line = document.createElement('div');
    line.className = `log-entry ${cls}`;
    line.textContent = `[${entry.timestamp}] [${entry.level}] ${entry.message}`;
    el.appendChild(line);
    if (el.children.length > 200) el.removeChild(el.firstChild);
    el.scrollTop = el.scrollHeight;
}

function updateMetrics(metric) {
    if (metric.packets !== undefined) document.getElementById('metric-packets').textContent = metric.packets;
}

fetchInitial();
</script>
</body>
</html>"#;
