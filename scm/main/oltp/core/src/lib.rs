//! `oltp_svc_core` — the technology-free reference implementation of
//! `oltp-pattern`'s `TransactionalStore` trait.
//!
//! [`InMemoryTransactionalStore`] holds records in a single
//! `std::sync::RwLock<HashMap<..>>` — no external storage technology, no
//! persistence, no distributed coordination. Records are lost if the
//! process exits.

mod in_memory_transactional_store;

pub use in_memory_transactional_store::InMemoryTransactionalStore;
