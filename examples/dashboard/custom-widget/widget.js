// Custom dashboard widget for UDS Dashboard
// Register with: uds.dashboard.registerWidget('my-widget', MyWidget)

class MyWidget {
    constructor(container) {
        this.container = container;
        this.container.innerHTML = '<h3>Custom Widget</h3><div id="widget-content">Loading...</div>';
    }

    async update(deviceId) {
        const response = await fetch(`/api/devices/${deviceId}/status`);
        const data = await response.json();
        document.getElementById('widget-content').textContent =
            `Device: ${data.name}, Uptime: ${data.uptime_seconds}s`;
    }
}

if (window.uds && window.uds.dashboard) {
    window.uds.dashboard.registerWidget('custom-status', MyWidget);
}
