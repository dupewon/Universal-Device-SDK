export interface DeviceInfo {
    id: string;
    name: string;
    platform: string;
    transport: string;
    connected: boolean;
    firmwareVersion?: string;
    uptimeSeconds?: number;
}

export interface UdsOptions {
    configPath?: string;
    transport?: string;
    device?: string;
    endpoint?: string;
}

export interface DeviceLog {
    timestamp: string;
    level: string;
    message: string;
}

export interface BenchmarkReport {
    type: string;
    minMs: number;
    maxMs: number;
    avgMs: number;
    p50Ms: number;
    p95Ms: number;
    p99Ms: number;
}

export class UdsClient {
    private options: UdsOptions;
    private ws: WebSocket | null = null;

    constructor(options: UdsOptions = {}) {
        this.options = options;
    }

    get endpoint(): string {
        return this.options.endpoint || 'http://localhost:4567';
    }

    get wsEndpoint(): string {
        return this.endpoint.replace(/^http/, 'ws') + '/ws';
    }

    async discover(timeoutMs = 5000): Promise<DeviceInfo[]> {
        const response = await fetch(`${this.endpoint}/api/devices`, {
            signal: AbortSignal.timeout(timeoutMs),
        });
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        return response.json();
    }

    async inspect(deviceId: string): Promise<DeviceInfo> {
        const response = await fetch(`${this.endpoint}/api/devices/${deviceId}`);
        if (!response.ok) throw new Error(`Device not found: ${deviceId}`);
        return response.json();
    }

    async flash(deviceId: string, firmware: Uint8Array): Promise<void> {
        const response = await fetch(`${this.endpoint}/api/devices/${deviceId}/flash`, {
            method: 'POST',
            body: firmware,
        });
        if (!response.ok) throw new Error(`Flash failed: ${response.statusText}`);
    }

    async logs(count = 100): Promise<DeviceLog[]> {
        const response = await fetch(`${this.endpoint}/api/logs?limit=${count}`);
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        return response.json();
    }

    async metrics() {
        const response = await fetch(`${this.endpoint}/api/metrics`);
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        return response.json();
    }

    connectWebSocket(): WebSocket {
        this.ws = new WebSocket(this.wsEndpoint);
        return this.ws;
    }

    disconnectWebSocket() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    onDeviceUpdate(callback: (devices: DeviceInfo[]) => void) {
        if (!this.ws) this.connectWebSocket();
        this.ws!.addEventListener('message', (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.type === 'device_update') callback(data.devices);
            } catch { }
        });
    }

    onLog(callback: (log: DeviceLog) => void) {
        if (!this.ws) this.connectWebSocket();
        this.ws!.addEventListener('message', (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.type === 'log') callback(data.entry);
            } catch { }
        });
    }

    async health(): Promise<{ status: string; version: string; timestamp: string }> {
        const response = await fetch(`${this.endpoint}/api/health`);
        return response.json();
    }
}
