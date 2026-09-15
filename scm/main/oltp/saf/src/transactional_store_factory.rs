//! [`TransactionalStoreFactory`] — public transactional-store construction surface.

use oltp_pattern::TransactionalStore;

/// Zero-size factory type for constructing transactional-store instances.
pub struct TransactionalStoreFactory;

impl TransactionalStoreFactory {
    /// Construct the in-process reference store, generic over the caller's
    /// own key/record types (e.g. `TransactionalStoreFactory::in_memory::<String, Order>()`).
    ///
    /// Zero-cost: returns `impl TransactionalStore` directly, not
    /// `Box<dyn TransactionalStore>` -- `TransactionalStore` isn't
    /// object-safe (it has associated types), so there is no boxed form to
    /// return even if one were wanted.
    ///
    /// Records are lost if the process exits -- no persistence, no
    /// distributed coordination. See
    /// [`oltp_svc_core::InMemoryTransactionalStore`]'s own doc comment.
    pub fn in_memory<K, R>() -> impl TransactionalStore<Key = K, Record = R>
    where
        K: Send + Sync + Clone + Eq + std::hash::Hash + 'static,
        R: Send + Sync + Clone + 'static,
    {
        oltp_svc_core::InMemoryTransactionalStore::<K, R>::new()
    }
}
