//! `oltp_svc_saf` — public construction surface for `oltp-svc`'s
//! `TransactionalStore` implementations.

mod transactional_store_factory;

pub use transactional_store_factory::TransactionalStoreFactory;
