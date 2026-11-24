# Rulekit Master Backlog & Work Tracker

**Document Version:** 1.0.0  
**Current Milestone:** `dc_rulekit` 0.2.0 Release & Stabilization  
**Repository:** [DevelopersCoffee/rulekit](https://github.com/DevelopersCoffee/rulekit)  
**Last Updated:** November 25, 2026  

---

## 1. Master Backlog Summary

| Task | Category | Current Status | Evidence / Reference | Next Action |
| :--- | :--- | :--- | :--- | :--- |
| **v0.1 Rust/Dart core engine** | Implementation | **MERGED & VERIFIED** | Commit `5f3bf6d`, Merge commit `794d5a9` (PR #1) | Close implementation thread |
| **v0.2 schema & API changes** | Implementation | **MERGED & VERIFIED** | Commit `999b431`, Merge commit `9b8989e` (PR #2) | Close implementation thread |
| **ADR 0001 & ADR 0002** | Architecture | **ACCEPTED & ALIGNED** | `docs/ADR/0001-agnostic-core-vs-host-plugins.md`, `0002-industry-standard-rule-documents.md` | Baseline maintained |
| **JSON Schema & Golden Fixtures** | Validation | **VERIFIED** | `schema/rule.schema.json`, `schema/fixtures/v*.json` | CI golden tests passing in Rust & Dart |
| **Rust Test Suite** | Quality | **PASSED** | `cargo test --all` (15/15 passed) | Re-run in CI / release commit |
| **Dart Analysis & Tests** | Quality | **PASSED** | `dart analyze` (0 issues), `dart test` (8/8 passed) | Re-run in CI / release commit |
| **Downstream Compatibility Mapping** | Compatibility | **COMPLETE** | `cruftkit-model-fetch/packages/rulekit_adapter` mapped | Cruftkit upgrade plan documented |
| **crates.io Publication (`dc_rulekit 0.2.0`)** | Release Operations | **PUBLISHED & VERIFIED** | Published Oct 3, 2026 by `ucguy4u`; verified via external smoke consumer test | Monitor telemetry and bug reports |
| **pub.dev Publication (`dc_rulekit 0.2.0`)** | Release Operations | **PACKAGED (Pending Auth)** | `dart pub publish --dry-run` passed with 0 warnings (8 KB archive) | Run `dart pub publish` using authorized account |
| **Published Package Smoke Testing** | Verification | **VERIFIED** | Clean consumer test compiled and resolved crate `0.2.0` | Test pub.dev after credential release |
| **Rust <-> Dart FFI Integration** | Architecture | **DEFERRED** | ADR 0001 (Deferred to keep on-device mobile builds simple) | Separate future ADR & spike |
| **Cron / Schedule Execution Runner** | Engine | **DEFERRED** | ADR 0001 (Host scheduler invokes evaluation) | Design standalone trigger runner |
| **Cruftkit Host-Specific Adapter** | Host Integration | **DEFERRED (Cruftkit Scope)**| Cruftkit repository `packages/rulekit_adapter` | Maintain clean separation from core |
| **Live Disk & Billing Integrations** | Domain Logic | **OUT OF SCOPE** | Core remains business-agnostic | Product-level implementation |
| **Domain-Specific UI / Rule Editors** | UI / Presentation | **OUT OF SCOPE** | Core remains headless | Implement in consuming applications |
| **Doc drift — ADR 0001 + baseline (PR #3)** | Documentation | **MERGED** | Merge `7e6de7f` ([PR #3](https://github.com/DevelopersCoffee/rulekit/pull/3)) | None |
| **Doc drift — main tip inventory (PR #4)** | Documentation | **MERGED** | Merge `9d7ad5b` ([PR #4](https://github.com/DevelopersCoffee/rulekit/pull/4)) | None |

### Status vocabulary

Terms below are used consistently across `RULEKIT_*` and `TODOS.md`:

| Label | Meaning |
| :--- | :--- |
| **MERGED** / **COMPLETED** | Landed on `main` |
| **VERIFIED** / **PASSED** | Recorded evidence (tests, registry smoke, dry-run) |
| **Pending** | Blocked on an external maintainer action (credentials, downstream repo) |
| **DEFERRED** | Intentionally future milestone per ADR |
| **OUT OF SCOPE** | Not planned for rulekit core |

---

## 2. Completed Work & Concrete Evidence

### 2.1 PR #1: v0.1 Core Engine (`794d5a9`)
- **Delivered Capabilities:**
  - Rust crate `dc_rulekit`: Rule model, proposal store, in-memory & file store, execution engine with dry-run, audit receipt hooks.
  - Dart package `dc_rulekit`: Mirror of core engine semantics for Flutter hosts.
  - Toy plugins: `dc_rulekit_demo_plugins` (`demo.when.*`, `demo.then.*`).
  - Architecture: ADR 0001 defining agnostic core vs host plugins.
- **Evidence:**
  - Commit `5f3bf6d` merged via `794d5a9` on October 2, 2026.
  - Publish metadata commit `fe4654c`.

### 2.2 PR #2: v0.2 Industry-Standard Rule Documents (`9b8989e`)
- **Delivered Capabilities:**
  - Rule document schema v2 (`schema_version: 2`): Nested boolean condition tree (`all`, `any`, `not`) aligned with `json-rules-engine`.
  - Event objects (`events[]` with `id`, `type`, `params`) replacing flat `then[]`.
  - Ingestion compatibility: Legacy v0.1 documents with `when`/`then` parse and normalize to v2 in memory.
  - Plugin parameter schema validation: Draft-07 JSON Schema validation during `ProposalStore::propose(rule, &registry)`.
  - Golden fixtures in `schema/fixtures/` and schema definition in `schema/rule.schema.json`.
  - Architecture: ADR 0002 accepted.
- **Evidence:**
  - Commit `999b431` merged via `9b8989e` on October 3, 2026.
  - Rust tests: 11 engine tests + 2 schema golden tests + 2 demo plugin tests passed.
  - Dart tests: 8 engine and fixture tests passed.

---

## 3. Active Release Tasks

### 3.1 Task R-01: Publish Dart Package to pub.dev
- **Priority:** High / Blocker for full multi-platform release.
- **Current State:** 
  - Code and metadata in `packages/dc_rulekit` are 100% prepared.
  - `pubspec.yaml` specifies version `0.2.0`, description, repository, and environment `sdk: ^3.5.0`.
  - `dart pub publish --dry-run` produces 0 warnings and an 8 KB archive.
- **Blocker:**
  - `dart pub publish` requires Google Account OAuth authentication in an interactive browser terminal with publishing rights for the `dc_rulekit` namespace on pub.dev.
- **Action Required:**
  - The authorized account maintainer (`coffee.devloper@gmail.com` / `DevelopersCoffee`) must execute `dart pub publish` from `packages/dc_rulekit`.

### 3.2 Task R-02: Downstream Cruftkit Upgrade Plan
- **Priority:** Medium.
- **Current State:**
  - Cruftkit workspace (`cruftkit-model-fetch/packages/rulekit_adapter`) pins `dc_rulekit: ^0.1.0`.
  - Cruftkit continues to build and run without disruption because SemVer protects `^0.1.0` from `0.2.0`.
- **Action Required:**
  - When Cruftkit schedules its v0.2 migration, apply the migration plan documented in `RULEKIT_RELEASE_PLAN.md` and upgrade `packages/rulekit_adapter/lib/src/bridge.dart` to use `conditions`, `events`, and `propose(rule, registry)`.

---

## 4. Explicitly Deferred Scope (Do Not Add to v0.2 Release)

These items were intentionally excluded from `dc_rulekit` 0.2.0 and must remain separate:

1. **Rust <-> Dart FFI (`flutter_rust_bridge` / `uniffi`):**
   - *Rationale:* Adds mobile toolchain and native compilation complexity without immediate performance necessity on current rule volumes. Pure Dart mirror provides identical behavior with zero native compilation overhead.
   - *Target:* Future milestone (separate ADR).
2. **Cron / Schedule Execution Runner:**
   - *Rationale:* ADR 0001 states that the host's background scheduler or OS alarms trigger rule evaluation; the engine only stores the `Trigger.schedule` expression string.
   - *Target:* Future trigger execution layer package.
3. **Cruftkit Host-Specific Adapter:**
   - *Rationale:* ADR 0001 mandates that product semantics (APFS, candidates, reclaim tiers) belong strictly in host repositories, not in `dc_rulekit`.
   - *Target:* Maintained in `DevelopersCoffee/cruftkit`.
4. **Live Disk and Billing Integrations:**
   - *Rationale:* Product-specific domain logic.
   - *Target:* Out of scope for rulekit core.
5. **Domain-Specific UI:**
   - *Rationale:* Visual rule builders and review shells are host-level components.
   - *Target:* Consuming product UI frameworks.
