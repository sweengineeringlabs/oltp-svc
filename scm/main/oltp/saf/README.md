# oltp-svc-saf

`TransactionalStoreFactory`: construction facade for every backend this
repo ships (currently just `in_memory::<K, R>()`, always available — no
feature gate). See [Architecture](../../../../docs/3-design/architecture.md)
for why this factory is a generic function returning zero-cost
`impl TransactionalStore`, not `Box<dyn TransactionalStore>`.
