from dataclasses import dataclass, field
from typing import Optional, Dict, List, Any
from .transport import TransportConfig


@dataclass
class DeviceCapability:
    name: str
    version: str
    available: bool = True
    config: Dict[str, Any] = field(default_factory=dict)


@dataclass
class Device:
    id: str
    name: str
    platform: str
    transport_hints: List[str]
    capabilities: Optional[Dict[str, DeviceCapability]] = None
    status: Optional[str] = None
    firmware_version: Optional[str] = None
    connected: bool = False
    transport_config: Optional[TransportConfig] = None


@dataclass
class DeviceInfo:
    id: str
    name: str
    platform: str
    firmware_version: str
    uptime_seconds: int
    connected: bool
    transport: str
    capabilities: Dict[str, bool]

    @classmethod
    def from_dict(cls, data: dict) -> "DeviceInfo":
        return cls(
            id=data.get("id", ""),
            name=data.get("name", ""),
            platform=data.get("platform", ""),
            firmware_version=data.get("firmware_version", ""),
            uptime_seconds=data.get("uptime_seconds", 0),
            connected=data.get("connected", False),
            transport=data.get("transport", ""),
            capabilities=data.get("capabilities", {}),
        )
