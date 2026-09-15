# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-in-memory-reference-implementation.md) | Accepted | 2026-09-15 | `InMemoryTransactionalStore` uses a single `std::sync::RwLock<HashMap<..>>`, not per-key locks or a dependency on a real database driver; no `spi` backend exists yet |

[← 3-design index](../README.md)
