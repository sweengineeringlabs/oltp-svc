# Glossary

Alphabetized list of terms used in `oltp-svc`.

---

**InMemoryTransactionalStore** - `oltp-svc-core`'s technology-free reference `TransactionalStore`: a single `std::sync::RwLock<HashMap<RecordKey, Record>>`. No persistence, no distributed coordination.

**TransactionalStoreFactory** - Construction facade in `oltp-svc-saf`: `in_memory`, returning `Box<dyn TransactionalStore>`.

[← Docs index](README.md)
