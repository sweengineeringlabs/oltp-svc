//! [`InMemoryTransactionalStore`] — records held in a single
//! `std::sync::RwLock<HashMap<..>>`, no external storage technology.

use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::sync::RwLock;

use oltp_pattern::{StoreError, TransactionalStore, WriteOp};

/// In-process [`TransactionalStore`] backed by a single
/// `std::sync::RwLock<HashMap<K, R>>`, generic over the caller's own
/// key/record types -- no forced serialization for an in-process consumer.
///
/// `write_batch` applies every op under one write-lock acquisition, so it
/// is atomic with respect to concurrent readers/writers of this same
/// instance (either all ops are visible, or none are — no in-between state
/// observable from another thread).
///
/// Records are lost if the process exits — no persistence, no distributed
/// coordination. That's the reference-impl trade-off; a persistent or
/// distributed backend is a `spi` crate this repo doesn't have yet (no
/// real consumer has needed one).
pub struct InMemoryTransactionalStore<K, R> {
    table: RwLock<HashMap<K, R>>,
}

impl<K, R> InMemoryTransactionalStore<K, R> {
    /// Construct a fresh store with no records.
    #[must_use]
    pub fn new() -> Self {
        Self {
            table: RwLock::new(HashMap::new()),
        }
    }
}

impl<K, R> Default for InMemoryTransactionalStore<K, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, R> TransactionalStore for InMemoryTransactionalStore<K, R>
where
    K: Send + Sync + Clone + Eq + Hash + 'static,
    R: Send + Sync + Clone + 'static,
{
    type Key = K;
    type Record = R;

    fn get(&self, key: &K) -> impl Future<Output = Result<Option<R>, StoreError>> + Send + '_ {
        let key = key.clone();
        async move {
            let table = self
                .table
                .read()
                .map_err(|_| StoreError::ConnectionFailed("lock poisoned".to_string()))?;
            Ok(table.get(&key).cloned())
        }
    }

    async fn put(&self, key: K, record: R) -> Result<(), StoreError> {
        let mut table = self
            .table
            .write()
            .map_err(|_| StoreError::WriteFailed("lock poisoned".to_string()))?;
        table.insert(key, record);
        Ok(())
    }

    fn delete(&self, key: &K) -> impl Future<Output = Result<(), StoreError>> + Send + '_ {
        let key = key.clone();
        async move {
            let mut table = self
                .table
                .write()
                .map_err(|_| StoreError::WriteFailed("lock poisoned".to_string()))?;
            table.remove(&key);
            Ok(())
        }
    }

    async fn write_batch(&self, ops: Vec<WriteOp<K, R>>) -> Result<(), StoreError> {
        let mut table = self
            .table
            .write()
            .map_err(|_| StoreError::TransactionFailed("lock poisoned".to_string()))?;
        for op in ops {
            match op {
                WriteOp::Put(k, r) => {
                    table.insert(k, r);
                }
                WriteOp::Delete(k) => {
                    table.remove(&k);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_transactional_store_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<InMemoryTransactionalStore<String, Vec<u8>>>();
        assert!(
            std::hint::black_box(true),
            "InMemoryTransactionalStore is Send + Sync (checked above at compile time)"
        );
    }
}
