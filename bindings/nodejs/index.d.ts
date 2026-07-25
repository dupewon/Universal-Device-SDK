/// <reference types="node" />
import { EventEmitter } from 'events';

export interface DeviceInfo {
    id: string;
    name: string;
    platform: string;
    transport: string;
    connected: boolean;
    firmware_version?: string;
    uptime_seconds?: number;
}

export interface LogOptions {
    level?: string;
    lines?: number;
}

export interface BenchmarkOptions {
    type?: 'latency' | 'throughput';
    duration?: number;
}

export declare class UdsClient extends EventEmitter {
    constructor(configPath?: string);
    discover(timeoutMs?: number): DeviceInfo[];
    inspect(deviceId: string): DeviceInfo;
    flash(deviceId: string, firmwarePath: string): void;
    logs(deviceId: string, options?: LogOptions): object[];
    rpc(deviceId: string, method: string, params?: string): object;
    startMonitoring(deviceId: string): void;
    stopMonitoring(): void;
    doctor(): object;
    benchmark(deviceId: string, options?: BenchmarkOptions): object;
    on(event: 'data', listener: (data: any) => void): this;
    on(event: 'end', listener: (code: number) => void): this;
}
