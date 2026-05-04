# nauron-contracts

Shared Rust contracts for Nauron services and worker messages.

This crate keeps service boundaries explicit. Producers and consumers use the same typed payloads for jobs, progress events, search requests, health responses, and condition evaluation responses instead of duplicating JSON shapes across repositories.

## What It Contains

| Area | Purpose |
| --- | --- |
| MIR | Document processing requests, progress events, results, stages, artifacts, and failure kinds. |
| RDF | RDF enrichment start messages, progress events, results, timings, and stage enums. |
| Ingest | Structured extraction start messages, progress events, results, schema fields, and token usage. |
| Conditions | Condition evaluation jobs, options, evidence, results, and validation helpers. |
| Vector | Semantic/vector search request and response DTOs. |
| Health | Service and component health response DTOs. |

## Install

```toml
[dependencies]
nauron-contracts = "0.1.0"
```

For local development:

```toml
[dependencies]
nauron-contracts = { path = "../nauron-contracts" }
```

Enable SQLx enum derives when a service maps contract enums directly to PostgreSQL enum columns:

```toml
[dependencies]
nauron-contracts = { version = "0.1.0", features = ["sqlx"] }
```

## Requirements

- Rust `1.95.0`
- Cargo

## Versioning

Every public queue payload carries `schema_version`. The initial value is `1`.

Compatibility rules:

| Change | Version impact |
| --- | --- |
| Add optional field | Patch or minor, depending on consumer impact. |
| Add enum variant | Minor if consumers can handle it explicitly. |
| Rename/remove field, rename enum variant, or change JSON shape | Major. |
| Change topic name or required field | Major. |

Unknown enum values are rejected during deserialization. Consumers should fail loudly instead of applying silent fallbacks.

## Example

```rust
use nauron_contracts::{MirRequest, OutputTarget, SchemaVersion, SourceRef};
use uuid::Uuid;

let request = MirRequest {
    schema_version: SchemaVersion::V1,
    job_id: Uuid::new_v4(),
    context_id: 42,
    user_id: None,
    source: SourceRef::S3 {
        bucket: "documents".into(),
        key: "uploads/spec.pdf".into(),
        version_id: None,
    },
    output: OutputTarget::new("artifacts", Some("jobs/job-123".into())),
    dry_run: false,
    attempt: 1,
    submitted_at: None,
};

let payload = serde_json::to_vec(&request)?;
```

## Development

```bash
cargo +1.95.0 fmt --check
python3 scripts/loc_check.py 250 rs
cargo +1.95.0 clippy --workspace --all-targets --all-features -- -D warnings
cargo +1.95.0 test --workspace --all-targets --all-features
```
