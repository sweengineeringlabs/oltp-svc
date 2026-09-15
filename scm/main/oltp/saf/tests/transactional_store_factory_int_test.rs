//! Integration tests for [`TransactionalStoreFactory`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use oltp_pattern::{Record, RecordKey};
use oltp_svc_saf::TransactionalStoreFactory;

/// @covers: TransactionalStoreFactory::in_memory — returns a real, working
/// store, not just a value of the right type.
#[tokio::test]
async fn test_in_memory_returns_a_working_store() {
    let store = TransactionalStoreFactory::in_memory();
    let key = RecordKey::new("k");
    store
        .put(key.clone(), Record::new(b"v".to_vec()))
        .await
        .expect("put must succeed");
    let got = store.get(&key).await.expect("get must succeed");
    assert_eq!(got.expect("record must exist").payload, b"v".to_vec());
}

/// @covers: TransactionalStoreFactory::in_memory — each call constructs an
/// independent store, not a shared singleton.
#[tokio::test]
async fn test_in_memory_constructs_independent_stores() {
    let store_a = TransactionalStoreFactory::in_memory();
    let store_b = TransactionalStoreFactory::in_memory();
    store_a
        .put(RecordKey::new("k"), Record::new(b"only-in-a".to_vec()))
        .await
        .expect("put must succeed");
    assert_eq!(
        store_b
            .get(&RecordKey::new("k"))
            .await
            .expect("get must succeed"),
        None
    );
}
