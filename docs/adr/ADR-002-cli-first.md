# ADR-002: CLI-First Architecture

**Status**: Accepted

**Context**: Many embedded tools require vendor IDEs or GUI applications. We want a toolchain that works in CI/CD pipelines and headless environments.

**Decision**: All functionality is exposed via `uds` CLI subcommands before any GUI, IDE plugin, or library wrapper is built. The CLI is the source of truth.

**Consequences**:
- Steeper initial learning curve for GUI-preferring users
- Clean integration with CI/CD, scripts, and automation
- Dashboard and IDE plugins are optional additions
- Machine-parseable output (JSON) is mandatory for all commands
