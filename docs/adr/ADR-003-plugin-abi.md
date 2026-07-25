# ADR-003: Stable Plugin ABI

**Status**: Accepted

**Context**: We want third-party developers to extend UDS without modifying core code or rebuilding from source.

**Decision**: Use a C-compatible dynamic library ABI with versioned symbols. Plugins are `.so`/`.dylib`/`.dll` files loaded at runtime via `libloading`. The ABI is versioned independently of UDS core.

**Consequences**:
- Language-agnostic plugin development (Rust, C, C++, Zig, etc.)
- Plugin isolation via dedicated threads
- Version compatibility checks at load time
- Cannot use Rust FFI directly; requires C ABI wrappers
