# Glossary

Alphabetized list of terms used in `oltp-svc`.

---

**InMemoryTransactionalStore\<K, R\>** - `oltp-svc-core`'s technology-free reference `TransactionalStore`: a single `std::sync::RwLock<HashMap<K, R>>`, generic over the caller's own key/record types. No persistence, no distributed coordination.

**TransactionalStoreFactory** - Construction facade in `oltp-svc-saf`: `in_memory::<K, R>()`, a generic function returning `impl TransactionalStore<Key = K, Record = R>` -- zero-cost, not `Box<dyn TransactionalStore>` (not possible since `TransactionalStore` has associated types).

[← Docs index](README.md)
