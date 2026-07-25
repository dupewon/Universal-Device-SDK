from enum import Enum
from typing import Optional, Callable
from dataclasses import dataclass, field


class TransportType(Enum):
    SERIAL = "serial"
    TCP = "tcp"
    UDP = "udp"
    WEBSOCKET = "websocket"
    BLE = "ble"
    USB = "usb"
    MOCK = "mock"


@dataclass
class TransportConfig:
    transport_type: TransportType
    path: Optional[str] = None
    baudrate: int = 115200
    host: Optional[str] = None
    port: Optional[int] = None
    timeout_ms: int = 5000
    extra: dict = field(default_factory=dict)


@dataclass
class TransportStats:
    bytes_sent: int = 0
    bytes_received: int = 0
    packets_sent: int = 0
    packets_received: int = 0
    errors: int = 0
    latency_ms: float = 0.0


class TransportConnection:
    def __init__(self, config: TransportConfig):
        self.config = config
        self._stats = TransportStats()
        self._connected = False
        self._on_data: Optional[Callable[[bytes], None]] = None

    def connect(self) -> bool:
        raise NotImplementedError

    def disconnect(self):
        self._connected = False

    def send(self, data: bytes) -> int:
        raise NotImplementedError

    def recv(self, size: int = 4096) -> bytes:
        raise NotImplementedError

    @property
    def connected(self) -> bool:
        return self._connected

    @property
    def stats(self) -> TransportStats:
        return self._stats

    def on_data(self, callback: Callable[[bytes], None]):
        self._on_data = callback
