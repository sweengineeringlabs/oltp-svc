//! [`InMemoryTransactionalStore`] — records held in a single
//! `std::sync::RwLock<HashMap<..>>`, no external storage technology.

use std::collections::HashMap;
use std::sync::RwLock;

use oltp_pattern::{Record, RecordKey, StoreError, StoreFuture, TransactionalStore, WriteOp};

/// In-process [`TransactionalStore`] backed by a single
/// `std::sync::RwLock<HashMap<RecordKey, Record>>`.
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
#[derive(Default)]
pub struct InMemoryTransactionalStore {
    table: RwLock<HashMap<RecordKey, Record>>,
}

impl InMemoryTransactionalStore {
    /// Construct a fresh store with no records.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransactionalStore for InMemoryTransactionalStore {
    fn get(&self, key: &RecordKey) -> StoreFuture<'_, Result<Option<Record>, StoreError>> {
        let key = key.clone();
        StoreFuture::new(async move {
            let table = self
                .table
                .read()
                .map_err(|_| StoreError::ConnectionFailed("lock poisoned".to_string()))?;
            Ok(table.get(&key).cloned())
        })
    }

    fn put(&self, key: RecordKey, record: Record) -> StoreFuture<'_, Result<(), StoreError>> {
        StoreFuture::new(async move {
            let mut table = self
                .table
                .write()
                .map_err(|_| StoreError::WriteFailed("lock poisoned".to_string()))?;
            table.insert(key, record);
            Ok(())
        })
    }

    fn delete(&self, key: &RecordKey) -> StoreFuture<'_, Result<(), StoreError>> {
        let key = key.clone();
        StoreFuture::new(async move {
            let mut table = self
                .table
                .write()
                .map_err(|_| StoreError::WriteFailed("lock poisoned".to_string()))?;
            table.remove(&key);
            Ok(())
        })
    }

    fn write_batch(&self, ops: Vec<WriteOp>) -> StoreFuture<'_, Result<(), StoreError>> {
        StoreFuture::new(async move {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_transactional_store_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<InMemoryTransactionalStore>();
        assert!(
            std::hint::black_box(true),
            "InMemoryTransactionalStore is Send + Sync (checked above at compile time)"
        );
    }
}
