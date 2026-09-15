//! Integration tests for [`oltp_svc_core::InMemoryTransactionalStore`].
//!
//! Picks `String`/`Vec<u8>` as the concrete `Key`/`Record` types for these
//! tests -- any real caller picks its own.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use oltp_pattern::{TransactionalStore, WriteOp};
use oltp_svc_core::InMemoryTransactionalStore;

fn store() -> InMemoryTransactionalStore<String, Vec<u8>> {
    InMemoryTransactionalStore::new()
}

/// @covers: InMemoryTransactionalStore::get — a key never written returns
/// `None`, not an error.
#[tokio::test]
async fn test_get_returns_none_for_unknown_key() {
    let store = store();
    assert_eq!(store.get(&"missing".to_string()).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore::put/get — a written record round-trips.
#[tokio::test]
async fn test_put_then_get_returns_the_written_record() {
    let store = store();
    let key = "order-1".to_string();
    store.put(key.clone(), b"pending".to_vec()).await.unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got, b"pending".to_vec());
}

/// @covers: InMemoryTransactionalStore::put — overwriting returns the new
/// value, not the old one.
#[tokio::test]
async fn test_put_overwrites_the_previous_value() {
    let store = store();
    let key = "order-1".to_string();
    store.put(key.clone(), b"pending".to_vec()).await.unwrap();
    store.put(key.clone(), b"paid".to_vec()).await.unwrap();
    let got = store.get(&key).await.unwrap().unwrap();
    assert_eq!(got, b"paid".to_vec());
}

/// @covers: InMemoryTransactionalStore::delete/get — a deleted key returns
/// `None` afterward.
#[tokio::test]
async fn test_get_after_delete_returns_none() {
    let store = store();
    let key = "order-1".to_string();
    store.put(key.clone(), b"paid".to_vec()).await.unwrap();
    store.delete(&key).await.unwrap();
    assert_eq!(store.get(&key).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore::delete — deleting a never-written
/// key succeeds rather than erroring.
#[tokio::test]
async fn test_delete_nonexistent_key_succeeds() {
    let store = store();
    assert!(store.delete(&"never".to_string()).await.is_ok());
}

/// @covers: InMemoryTransactionalStore::write_batch — atomicity under a
/// single lock: every op in the batch is visible after the call returns.
#[tokio::test]
async fn test_write_batch_applies_every_op_atomically() {
    let store = store();
    store
        .put("stale".to_string(), b"old".to_vec())
        .await
        .unwrap();

    store
        .write_batch(vec![
            WriteOp::Put("a".to_string(), b"1".to_vec()),
            WriteOp::Put("b".to_string(), b"2".to_vec()),
            WriteOp::Delete("stale".to_string()),
        ])
        .await
        .unwrap();

    assert_eq!(
        store.get(&"a".to_string()).await.unwrap().unwrap(),
        b"1".to_vec()
    );
    assert_eq!(
        store.get(&"b".to_string()).await.unwrap().unwrap(),
        b"2".to_vec()
    );
    assert_eq!(store.get(&"stale".to_string()).await.unwrap(), None);
}

/// @covers: InMemoryTransactionalStore -- two independent instances do not
/// share state (each store owns its own table).
#[tokio::test]
async fn test_two_instances_do_not_share_state() {
    let store_a = store();
    let store_b = store();
    store_a
        .put("k".to_string(), b"only-in-a".to_vec())
        .await
        .unwrap();
    assert_eq!(store_b.get(&"k".to_string()).await.unwrap(), None);
}
