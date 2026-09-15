# oltp-svc-core

`InMemoryTransactionalStore`: the technology-free reference implementation
of `oltp-pattern`'s `TransactionalStore` trait — a single
`std::sync::RwLock<HashMap<..>>`, no persistence, no distributed
coordination.

See [Architecture](../../../../docs/3-design/architecture.md) for the full
explanation.
