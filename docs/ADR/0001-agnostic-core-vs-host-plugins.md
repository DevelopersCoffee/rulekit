# ADR 0001: Agnostic core vs host plugins

## Status

Accepted (v0.1.0)

## Context

DevelopersCoffee needs an on-device rules framework usable across multiple apps (Flutter hosts, future native tools). Product-specific semantics (storage cleanup, billing, etc.) must not leak into the shared engine. Crates.io names `rulekit` and `rule_kit` are taken; the published Rust crate is **`dc_rulekit`**, aligned with the Dart package **`dc_rulekit`**. The GitHub repository remains **`DevelopersCoffee/rulekit`**.

## Decision

### Agnostic core

The core exposes only:

- **Rule document model**: `Rule`, `Condition`, `Action`, `Trigger` (`manual` | `schedule` | `event`), `RuleSource` (`static` | `llm`), `schema_version`.
- **Lifecycle**: `ProposalStore` (`propose` → `approve` / `reject`) → durable **active** rules in `RuleStore`.
- **Evaluation**: `Engine` evaluates `when` conditions in order (all must pass; fail-fast), then runs `then` actions in order.
- **Extensions**: `PluginRegistry` maps string plugin ids to host-supplied `ConditionEvaluator` / `ActionHandler`.
- **Audit**: `AuditHook` receives an `AuditReceipt` with structured outcomes plus an **opaque** JSON map the host interprets.
- **Dry-run**: Impure actions (`is_pure == false`) are not executed; outcomes record `skipped_dry_run`. Pure actions may run under dry-run.

The engine never performs domain effects; it only calls registered plugins.

### Fail-closed sandbox

If a rule references an unknown condition or action plugin id, evaluation fails before side effects (`UnknownConditionPlugin` / `UnknownActionPlugin`). Disabled rules reject evaluation (`DisabledRule`). Schema version mismatches reject load/evaluate (`SchemaVersionMismatch`).

### Multi-app namespacing

Rule ids and plugin ids are opaque strings. Conventions (not enforced by the engine): `app.namespace/rule-id` and `app.when.*` / `app.then.*`. Toy plugins use the `demo.*` prefix in `dc_rulekit_demo_plugins`.

### Rust + Dart split (v0.1)

| Layer | Choice |
|-------|--------|
| Canonical on-device core | **Rust** crate `dc_rulekit` (publishable to crates.io) |
| Flutter / Dart hosts | **Pure Dart** package `dc_rulekit` mirroring schema and semantics |
| FFI | **Deferred** — `flutter_rust_bridge` / `uniffi` adds build complexity for v0.1; ADR revisits when a single binary core is required on mobile |

Both implementations share the same JSON rule shape and error semantics so hosts can migrate to FFI later without changing rule documents.

### Schema versioning

`schema_version` on each rule must match `CURRENT_SCHEMA_VERSION` (Rust) / `currentSchemaVersion` (Dart). Bump the constant and migration story when breaking JSON fields.

### Scheduling

`Trigger.schedule` stores a host-defined expression string. **No cron runner** in v0.1; the host scheduler invokes evaluation.

## Consequences

- Cruftkit and other products are **consumers** only; no Cruftkit adapter in the core repo.
- Demo plugins live in `crates/dc_rulekit_demo_plugins` and Dart tests inline toy plugins — not shipped as production domain logic.
- Documentation and README refer to the published name **`dc_rulekit`** while linking to the **`rulekit`** GitHub repo.
