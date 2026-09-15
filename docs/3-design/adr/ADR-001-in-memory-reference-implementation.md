# ADR-001: `InMemoryTransactionalStore`'s implementation shape

**Status**: Accepted
**Date**: 2026-09-15

## Context

`oltp-pattern` has no existing pilot (see that repo's own ADR-001), so this
crate's implementation choices are new design, not extraction. Two real
alternatives were considered for how `InMemoryTransactionalStore` holds and
guards its records:

1. **A single `std::sync::RwLock<HashMap<K, R>>`** (chosen, `K`/`R` being
   whatever key/record types the caller picks via `TransactionalStore`'s
   own `Key`/`Record` associated types): one lock guards the whole table.
   Reads take a shared (`read`) lock, writes (including `write_batch`)
   take an exclusive (`write`) lock for the whole operation.
2. **Per-key locking** (e.g. a `HashMap<K, RwLock<R>>`, or a sharded lock
   table): finer-grained concurrency, but `write_batch` would
   need to acquire multiple per-key locks in a consistent order to stay
   atomic and avoid deadlock against a concurrent `write_batch` touching
   an overlapping key set.

## Decision

Chose (1), a single `RwLock` over the whole table, for this first version:

- `write_batch` atomicity falls out for free: one write-lock acquisition
  covers every op in the batch, so no other reader or writer can observe a
  partially-applied batch. Per-key locking would need real multi-key
  lock-ordering discipline to get the same guarantee.
- Simpler to implement and verify correctly — no lock-ordering protocol to
  get right, no deadlock surface between concurrent batches.
- Honest, known limitation: every write blocks every other write and every
  read for the whole table, not just the keys involved — this does not
  scale to high write-concurrency across unrelated keys. If a real
  consumer needs that, a real database backend (a `spi` crate) is the
  right fix, not a hand-rolled per-key locking scheme in `core` — same
  reasoning `scheduler-svc`'s own ADR-001 gives for not building a shared
  timer wheel speculatively.

## Consequences

- `oltp-svc-core` depends on `oltp-pattern` only — no `futures` dependency
  (unlike `scheduler-svc-core`), since every operation here is genuinely
  synchronous work (a `HashMap` lookup/insert/remove under a lock, no
  actual I/O), and `TransactionalStore`'s own methods return `impl Future`
  (RPITIT, no `futures` crate involved) rather than a boxed future, purely
  to satisfy the trait's async shape, not because this backend performs
  any real asynchronous work.
- `InMemoryTransactionalStore` does not implement `Clone` (unlike
  `message-broker-svc-core`'s `InMemoryMessageBroker`) — `oltp-svc-saf`
  constructs exactly one instance per `TransactionalStoreFactory::in_memory()`
  call and hands it back directly (as `impl TransactionalStore`, not a
  boxed trait object — see architecture.md); there is no need for multiple
  handles sharing one table the way pub/sub subscribers need to share one
  broker's channel map.
