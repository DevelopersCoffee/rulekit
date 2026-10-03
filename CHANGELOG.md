# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-10-02

### Changed (breaking)

- Rule document **v2** (`schema_version: 2`): nested `conditions` (`all` / `any` / `not`) instead of flat `when[]`; `events[]` with `{ id, type, params }` instead of `then[]` with `plugin`.
- Rust/Dart APIs: `Rule.conditions`, `Rule.events`, `RuleEvent`, `ConditionNode`; `ProposalStore::propose(rule, registry)` validates plugin params against optional JSON Schema.
- Audit `action_outcomes` use `type` (event type) instead of `plugin`.

### Added

- ADR 0002 — alignment with json-rules-engine / JSON Logic conventions.
- `schema/rule.schema.json` and golden fixtures; Rust CI tests validate fixtures against schema.
- Optional `params_schema()` on condition/action plugins; fail-closed validation at propose time (Rust + Dart: draft-07 subset validator).
- v0.1 JSON ingest compat: `when` / `then` / `plugin` normalize to v2 on read.

### Migration

| v0.1 | v0.2 |
|------|------|
| `"schema_version": 1` | `"schema_version": 2` |
| `"when": [ { "id", "plugin", "params" } ]` | `"conditions": { "all": [ ... ] }` |
| `"then": [ { "id", "plugin", "params" } ]` | `"events": [ { "id", "type", "params" } ]` |
| `proposals.propose(rule)` | `proposals.propose(rule, &registry)` |

Cruftkit and other hosts pinned to `^0.1.0` keep working on crates.io/pub.dev **0.1.x** until they opt into `0.2`.

## [0.1.0] - 2026-10-02

### Added

- Rust crate **`dc_rulekit`**: rule model, plugin registry, in-memory/file stores, proposal workflow, engine with dry-run, audit hook.
- Dart package **`dc_rulekit`**: pure-Dart mirror of core semantics for Flutter hosts.
- Toy plugins in **`dc_rulekit_demo_plugins`** (`demo.when.*`, `demo.then.*`).
- ADR 0001 (agnostic core vs host plugins).
- Unit tests for Rust and Dart.
