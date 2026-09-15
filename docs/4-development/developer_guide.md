# oltp-svc Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
oltp-svc/
├── README.md
├── docs/
│   ├── README.md
│   ├── glossary.md
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-in-memory-reference-implementation.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # workspace: [core, saf] -- no spi/ yet
    └── main/oltp/
        ├── core/              # oltp-svc-core -- InMemoryTransactionalStore (technology-free)
        └── saf/               # oltp-svc-saf -- TransactionalStoreFactory
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Not yet published to crates.io. First publish will be v0.1.0 for both crates.
- Depends on [`oltp-pattern`](https://github.com/sweengineeringlabs/oltp-pattern)
  by `git`+`tag` (`tag = "v0.1.0"`), this org's standing convention for a
  cross-repo dependency whose target hasn't published to crates.io yet.
  Switch to a version requirement once `oltp-pattern` publishes.

## Working on Any Crate

Both crates are members of `scm/Cargo.toml`, so from `scm/`:

```
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

`TransactionalStoreFactory::in_memory()` is always available, no feature
required — only one backend exists.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-in-memory-reference-implementation.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
