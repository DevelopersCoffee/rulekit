# ADR 0002: Industry-standard rule document shapes

## Status

Accepted (2026-10-02)

## Context

`dc_rulekit` v0.1 used a flat `when` array and action objects with a `plugin` field. That shape is easy to implement but unfamiliar to LLMs and humans who already know [json-rules-engine](https://github.com/CacheControl/json-rules-engine) and [JSON Logic](https://jsonlogic.com/) conventions.

Hosts (including Cruftkit) pin `dc_rulekit` `^0.1.0`; we must ship v0.2 without removing v0.1 from registries.

## Decision

**Rule JSON v2 (`schema_version: 2`)** aligns with industry patterns:

| Concept | v0.1 | v0.2 (standard-aligned) |
|--------|------|-------------------------|
| Conditions | Flat `when[]` | Nested tree under `conditions` with `all` / `any` / `not` |
| Condition leaf | `{ id, plugin, params }` | Same; `fact` accepted as alias for `plugin` on ingest |
| Actions | `then[]` with `plugin` | `events[]` with `{ id, type, params }` (json-rules-engine event shape) |
| Context | `EvalContext.facts` + optional `event` | Unchanged — plain JSON bags |

**Unchanged differentiators** (not part of the rule doc standard):

- Proposal → approve → active lifecycle
- Dry-run and pure vs impure actions
- Fail-closed plugin registry
- Opaque audit receipts

**Compatibility**

- Deserialize accepts `schema_version` 1 or 2; v1 documents normalize to v2 in memory (`when` → `conditions.all`, `then` → `events` with `type` ← `plugin`).
- New writes and `validate_schema()` require v2.
- Optional JSON Schema on plugins; params validated at **propose** time when a schema is registered.

**Future**

- Condition nodes may gain a JSON Logic operator object (`{ "==": [...] }`) alongside plugin leaves without breaking v2 trees.

## Consequences

- Breaking Rust/Dart API: `Rule.when` / `Rule.then` → `Rule.conditions` / `Rule.events`; `propose(rule, registry)` validates params.
- Cruftkit on `^0.1.0` is unaffected until it opts into `0.2`.
- `schema/rule.schema.json` and golden fixtures guard Rust/Dart parity in CI.

## References

- [json-rules-engine conditions](https://github.com/CacheControl/json-rules-engine/blob/master/docs/rules.md)
- ADR 0001 — agnostic core vs host plugins
