# oltp-svc

> **TLDR:** The in-process `InMemoryTransactionalStore`, on top of
> [`oltp-pattern`](https://github.com/sweengineeringlabs/oltp-pattern)'s
> contract. See [Architecture](docs/3-design/architecture.md) for the full design.

Companion implementation repo to
[`oltp-pattern`](https://github.com/sweengineeringlabs/oltp-pattern) — see
that repo's own ADR-001 (and its amendment) for why this domain was
designed contract-first, and for the real prior art found afterward that
`TransactionalStore` is now shaped around.

## Quick Start

```rust
use oltp_pattern::TransactionalStore;
use oltp_svc_saf::TransactionalStoreFactory;

async fn store_and_fetch() {
    let store = TransactionalStoreFactory::in_memory::<String, Vec<u8>>();
    let key = "order-42".to_string();
    store.put(key.clone(), b"paid".to_vec()).await.unwrap();
    let record = store.get(&key).await.unwrap();
    assert_eq!(record.unwrap(), b"paid".to_vec());
}
```

## Crates

| Crate | What it is |
|-------|------------|
| [`oltp-svc-core`](scm/main/oltp/core) | The technology-free reference implementation: `InMemoryTransactionalStore` (single `RwLock<HashMap<..>>`, no persistence) |
| [`oltp-svc-saf`](scm/main/oltp/saf) | `TransactionalStoreFactory` — construction facade consumers depend on |

No `spi` crate yet — no real consumer has needed a real transactional
database backend. See [Architecture](docs/3-design/architecture.md).

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, why `TransactionalStoreFactory` is a generic function, not a boxed-return factory |
| [ADR-001](docs/3-design/adr/ADR-001-in-memory-reference-implementation.md) | Why `InMemoryTransactionalStore` uses a single `RwLock`, not per-key locks |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, working on this crate |

## License

MIT OR Apache-2.0
