import subprocess
import json
from typing import List, Optional
from .device import Device


class UdsClient:
    def __init__(self, config_path: Optional[str] = None):
        self._config_path = config_path

    def discover(self, timeout_ms: int = 5000) -> List[Device]:
        args = ["uds", "devices", "--scan", "--output", "json"]
        if self._config_path:
            args.extend(["--config", self._config_path])
        result = subprocess.run(args, capture_output=True, text=True, timeout=timeout_ms / 1000)
        if result.returncode != 0:
            raise RuntimeError(f"UDS error: {result.stderr}")
        devices = json.loads(result.stdout)
        return [Device(**d) for d in devices]

    def flash(self, device_id: str, firmware_path: str) -> None:
        args = ["uds", "flash", "--device", device_id, firmware_path]
        subprocess.run(args, check=True)

    def monitor(self, device_id: str) -> None:
        args = ["uds", "monitor", "--device", device_id]
        subprocess.run(args, check=True)
