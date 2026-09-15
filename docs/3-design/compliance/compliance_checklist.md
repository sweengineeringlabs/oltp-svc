# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. `core` is technology-free

| # | Rule | Verify |
|---|------|--------|
| 1 | `oltp-svc-core` names no storage technology and has no external dependency beyond `oltp-pattern` | `grep -nE "^\s*(pub )?(struct\|enum\|fn) \w*(Postgres\|MySql\|Sqlite\|Redis\|DynamoDb)" main/oltp/core/src/*.rs` returns nothing; `main/oltp/core/Cargo.toml`'s `[dependencies]` lists only `oltp-pattern` |

## 2. `TransactionalStoreFactory` returns `Box<dyn TransactionalStore>` uniformly

| # | Rule | Verify |
|---|------|--------|
| 2 | `TransactionalStoreFactory::in_memory` returns `Box<dyn TransactionalStore>` | `grep -n "Box<dyn TransactionalStore>" main/oltp/saf/src/*.rs` shows the constructor's return type |
| 3 | `saf`'s own `lib.rs` never re-exports a concrete backend type (`InMemoryTransactionalStore`) | `grep -n "^pub use" main/oltp/saf/src/lib.rs` shows only `TransactionalStoreFactory` |

## 3. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 4 | `#![deny(unsafe_code)]` enforced across every crate | `cargo build --workspace` fails on any `unsafe` block |
| 5 | `#![warn(missing_docs)]` enforced across every crate | `cargo doc --workspace --no-deps` warns on any undocumented public item |
| 6 | `cargo clippy --workspace --all-targets -- -D warnings` clean | Run before every commit |
| 7 | `cargo fmt --check` clean across every crate | Run before every commit |

[← 3-design index](../README.md)
