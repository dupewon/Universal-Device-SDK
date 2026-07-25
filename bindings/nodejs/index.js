const { execSync, spawn } = require('child_process');
const EventEmitter = require('events');

class UdsClient extends EventEmitter {
    constructor(configPath) {
        super();
        this.configPath = configPath;
        this._monitorProcess = null;
    }

    _buildArgs(cmd) {
        const args = ['uds', ...cmd.split(' ')];
        if (this.configPath) args.push('--config', this.configPath);
        return args;
    }

    _execJson(cmd, timeoutMs = 10000) {
        const args = this._buildArgs(cmd + ' --output json');
        const out = execSync(args.join(' '), { timeout: timeoutMs, encoding: 'utf-8' });
        return JSON.parse(out.trim());
    }

    discover(timeoutMs = 5000) {
        return this._execJson('devices --scan', timeoutMs);
    }

    inspect(deviceId) {
        return this._execJson(`inspect --device ${deviceId}`);
    }

    flash(deviceId, firmwarePath) {
        execSync(`uds flash --device ${deviceId} ${firmwarePath}`, { stdio: 'inherit' });
    }

    logs(deviceId, options = {}) {
        const { level, lines = 50 } = options;
        let cmd = `logs --device ${deviceId} --lines ${lines}`;
        if (level) cmd += ` --level ${level}`;
        return this._execJson(cmd);
    }

    rpc(deviceId, method, params = '{}') {
        return this._execJson(`rpc --device ${deviceId} --method ${method} --params '${params}'`);
    }

    startMonitoring(deviceId) {
        if (this._monitorProcess) this.stopMonitoring();
        const args = this._buildArgs(`monitor --device ${deviceId} --output json`);
        this._monitorProcess = spawn(args[0], args.slice(1), { stdio: ['ignore', 'pipe', 'pipe'] });

        let buffer = '';
        this._monitorProcess.stdout.on('data', (chunk) => {
            buffer += chunk.toString();
            const lines = buffer.split('\n');
            buffer = lines.pop();
            for (const line of lines) {
                if (line.trim()) {
                    try { this.emit('data', JSON.parse(line)); }
                    catch { this.emit('data', line); }
                }
            }
        });

        this._monitorProcess.on('exit', (code) => {
            this._monitorProcess = null;
            this.emit('end', code);
        });
    }

    stopMonitoring() {
        if (this._monitorProcess) {
            this._monitorProcess.kill();
            this._monitorProcess = null;
        }
    }

    doctor() {
        return this._execJson('doctor');
    }

    benchmark(deviceId, options = {}) {
        const { type = 'latency', duration = 10 } = options;
        return this._execJson(`benchmark --device ${deviceId} --type ${type} --duration ${duration}`);
    }
}

module.exports = { UdsClient };
