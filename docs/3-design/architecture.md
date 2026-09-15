# oltp-svc Architecture

**Audience**: Architects, technical leads, contributors.

## Overview

Two crates — one `core` reference implementation, one `saf` facade. No
`spi` crate yet:

- **`oltp-svc-core`** — the technology-free reference implementation:
  `InMemoryTransactionalStore` (a single `std::sync::RwLock<HashMap<..>>`,
  no persistence, no distributed coordination). See
  [ADR-001](adr/ADR-001-in-memory-reference-implementation.md) for the
  implementation-shape reasoning.
- **`oltp-svc-saf`** — `TransactionalStoreFactory` (`in_memory`, always
  available — no feature gate, since there's only one backend). A consumer
  depends on `oltp-pattern` + `oltp-svc-saf` alone.

## `TransactionalStore` is object-safe — matches `MessageBroker`/`Scheduler`

`TransactionalStore`'s methods (`get`/`put`/`delete`/`write_batch`) have no
generic parameters, so it is fully object-safe: `Box<dyn TransactionalStore>`
exists. `TransactionalStoreFactory::in_memory()` returns
`Box<dyn TransactionalStore>` uniformly, matching `message-broker-svc-saf`'s
`MessageBrokerFactory` and `scheduler-svc-saf`'s `SchedulerFactory` shape —
unlike `executor-svc-saf`'s `ExecutorFactory`, which must return
`impl Executor` per constructor because `Executor::run<F: Future>` is
generic.

## Component Diagram

```mermaid
flowchart TD
    subgraph pattern["oltp-pattern"]
        contract["TransactionalStore, RecordKey, Record, WriteOp, StoreError"]
    end

    subgraph svc["oltp-svc"]
        core["oltp-svc-core<br/>InMemoryTransactionalStore"]
        saf["oltp-svc-saf<br/>TransactionalStoreFactory"]

        core -->|implements| contract
        saf -->|wires| core
    end
```

## Why a single `RwLock`, not per-key locks

See [ADR-001](adr/ADR-001-in-memory-reference-implementation.md). One
`RwLock<HashMap<RecordKey, Record>>` guarding the whole table means
`write_batch` is trivially, correctly atomic — every op in the batch
applies under one write-lock acquisition, so no other reader or writer can
observe a partially-applied batch. Per-key locking would need real
multi-key locking discipline (consistent lock ordering to avoid deadlock)
to get the same atomicity guarantee for `write_batch`, for a concurrency
win that no real consumer has asked for yet.

## Why no `spi` crate yet

No real consumer has needed a persistent (survives process restart) or
distributed (coordinates across multiple processes) OLTP backend. Per this
org's own `<domain>-<technology>-spi` convention, an `spi` crate wraps
exactly one external technology — building one speculatively, with no real
technology chosen and no real consumer validating the choice, would be the
same premature-generalization mistake `oltp-pattern`'s own ADR-001 declined
to make for secondary indexes/versioned writes/TTL. Add one when a real
need does.

## Scope boundary

This repo implements exactly `oltp-pattern`'s `TransactionalStore` trait,
one backend. Not covered, deliberately:

- **Persistent/distributed OLTP backends** — no `spi` crate yet, see above.
- **Per-key locking / finer-grained concurrency** — considered and
  deferred, see ADR-001's own reasoning.
- **Declarative queries over the whole dataset** — that's `olap-svc`'s
  `AnalyticalStore` implementation, a separate companion repo pair.

[← Docs index](../README.md)
