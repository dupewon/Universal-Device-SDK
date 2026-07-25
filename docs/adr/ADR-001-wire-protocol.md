# ADR-001: Binary Wire Protocol

**Status**: Accepted

**Context**: We need a protocol that works on constrained devices (RAM < 2 KB, slow CPUs) while supporting rich RPC semantics.

**Decision**: Use a custom binary, length-delimited frame format with:
- 12-byte header (magic, version, flags, sequence, length, checksum)
- CRC16-CCITT for error detection
- Optional AES-256-GCM / ChaCha20-Poly1305 encryption
- Protobuf + FlatBuffers as dual serialization backends

**Consequences**:
- Minimal overhead (≤ 8 bytes per message)
- No parser dependencies on device side
- Encryption adds ~28 bytes per frame
