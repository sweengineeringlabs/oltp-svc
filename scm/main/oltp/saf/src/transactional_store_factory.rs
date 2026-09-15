//! [`TransactionalStoreFactory`] — public transactional-store construction surface.

use oltp_pattern::TransactionalStore;

/// Zero-size factory type for constructing transactional-store instances.
pub struct TransactionalStoreFactory;

impl TransactionalStoreFactory {
    /// Construct the in-process reference store.
    ///
    /// Records are lost if the process exits -- no persistence, no
    /// distributed coordination. See
    /// [`oltp_svc_core::InMemoryTransactionalStore`]'s own doc comment.
    pub fn in_memory() -> Box<dyn TransactionalStore> {
        Box::new(oltp_svc_core::InMemoryTransactionalStore::new())
    }
}
