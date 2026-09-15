//! Integration tests for [`oltp_svc_core::InMemoryTransactionalStore`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use oltp_pattern::{Record, RecordKey, TransactionalStore, WriteOp};
use oltp_svc_core::InMemoryTransactionalStore;

/// @covers: InMemoryTransactionalStore::get — a key never written returns
/// `None`, not an error.
#[tokio::test]
async fn test_get_returns_none_for_unknown_key() {
    let store = InMemoryTransactionalStore::new();
    assert_eq!(store.get(&RecordKey::new("missing")).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore::put/get — a written record round-trips.
#[tokio::test]
async fn test_put_then_get_returns_the_written_record() {
    let store = InMemoryTransactionalStore::new();
    let key = RecordKey::new("order-1");
    store
        .put(key.clone(), Record::new(b"pending".to_vec()))
        .await
        .unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got.payload, b"pending".to_vec());
}

/// @covers: InMemoryTransactionalStore::put — overwriting returns the new
/// value, not the old one.
#[tokio::test]
async fn test_put_overwrites_the_previous_value() {
    let store = InMemoryTransactionalStore::new();
    let key = RecordKey::new("order-1");
    store
        .put(key.clone(), Record::new(b"pending".to_vec()))
        .await
        .unwrap();
    store
        .put(key.clone(), Record::new(b"paid".to_vec()))
        .await
        .unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got.payload, b"paid".to_vec());
}

/// @covers: InMemoryTransactionalStore::delete/get — a deleted key returns
/// `None` afterward.
#[tokio::test]
async fn test_get_after_delete_returns_none() {
    let store = InMemoryTransactionalStore::new();
    let key = RecordKey::new("order-1");
    store
        .put(key.clone(), Record::new(b"paid".to_vec()))
        .await
        .unwrap();
    store.delete(&key).await.unwrap();
    assert_eq!(store.get(&key).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore::delete — deleting a never-written
/// key succeeds rather than erroring.
#[tokio::test]
async fn test_delete_nonexistent_key_succeeds() {
    let store = InMemoryTransactionalStore::new();
    assert!(store.delete(&RecordKey::new("never")).await.is_ok());
}

/// @covers: InMemoryTransactionalStore::write_batch — atomicity under a
/// single lock: every op in the batch is visible after the call returns.
#[tokio::test]
async fn test_write_batch_applies_every_op_atomically() {
    let store = InMemoryTransactionalStore::new();
    store
        .put(RecordKey::new("stale"), Record::new(b"old".to_vec()))
        .await
        .unwrap();

    store
        .write_batch(vec![
            WriteOp::Put(RecordKey::new("a"), Record::new(b"1".to_vec())),
            WriteOp::Put(RecordKey::new("b"), Record::new(b"2".to_vec())),
            WriteOp::Delete(RecordKey::new("stale")),
        ])
        .await
        .unwrap();

    assert_eq!(
        store
            .get(&RecordKey::new("a"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"1".to_vec()
    );
    assert_eq!(
        store
            .get(&RecordKey::new("b"))
            .await
            .unwrap()
            .unwrap()
            .payload,
        b"2".to_vec()
    );
    assert_eq!(store.get(&RecordKey::new("stale")).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore -- two independent instances do not
/// share state (each store owns its own table).
#[tokio::test]
async fn test_two_instances_do_not_share_state() {
    let store_a = InMemoryTransactionalStore::new();
    let store_b = InMemoryTransactionalStore::new();
    store_a
        .put(RecordKey::new("k"), Record::new(b"only-in-a".to_vec()))
        .await
        .unwrap();
    assert_eq!(store_b.get(&RecordKey::new("k")).await.unwrap(), None);
}
