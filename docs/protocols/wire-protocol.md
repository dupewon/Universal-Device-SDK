# UDS Wire Protocol

## Frame Format

```
+------------------+------------------+------------------+------------------+
| Magic (4 bytes)  | Version (1 byte) | Flags (1 byte)   | Sequence (2 bytes)|
+------------------+------------------+------------------+------------------+
| Payload Length (2 bytes) | Checksum (2 bytes) | Payload (variable)  |
+------------------+------------------+------------------+------------------+
```

- **Magic**: `0x55 0x44 0x53 0x21` ("UDS!")
- **Version**: Major protocol version
- **Flags**: Request/response, streaming, compression bits
- **Sequence**: 16-bit monotonic counter for request-response correlation
- **Payload Length**: 16-bit, max 65535 bytes
- **Checksum**: CRC16-CCITT over payload
- **Payload**: Encoded message (Protobuf or FlatBuffers)

## Handshake

1. Client sends Magic + Version + empty frame
2. Server responds with version + capabilities
3. Highest common version negotiated
4. Optional encryption handshake (key exchange)

## Message Types

- **Request**: Method name + parameters + timeout
- **Response**: Status code + result + error message
- **Notification**: Fire-and-forget message
- **StreamHeader / StreamData / StreamEnd**: Streaming messages
