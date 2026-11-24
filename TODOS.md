# TODOS & Release Verification Evidence

This file tracks the status of all implementation, release, and deferred tasks for `dc_rulekit`, with concrete verification evidence.

**Status labels** match `RULEKIT_BACKLOG.md`: **COMPLETED** / **VERIFIED** = done with evidence; unchecked `[ ]` = **Pending** (external blocker) or future work; **Deferred** = ADR-excluded scope.

---

## 1. Completed Implementation Tasks

### [x] v0.1 Core Rules Engine (Rust + Dart Mirror)
- **Status:** COMPLETED & MERGED
- **PR:** [#1](https://github.com/DevelopersCoffee/rulekit/pull/1) (Merged 2026-10-02, commit `794d5a9`, feature commit `5f3bf6d`)
- **Evidence:**
  - Rust crate `crates/dc_rulekit`: Implemented `Rule`, `Condition`, `Action`, `Engine`, `EvalContext`, `ProposalStore`, `RuleStore`.
  - Dart mirror `packages/dc_rulekit`: Implemented mirror core in pure Dart for Flutter hosts.
  - Demo plugins `crates/dc_rulekit_demo_plugins`: Implemented toy condition/action plugins (`demo.when.*`, `demo.then.*`).
  - Architecture: Documented in `docs/ADR/0001-agnostic-core-vs-host-plugins.md`.
  - Packaging: Metadata added in commit `fe4654c`.

### [x] v0.2 Industry-Standard Rule Documents & API Alignment
- **Status:** COMPLETED & MERGED
- **PR:** [#2](https://github.com/DevelopersCoffee/rulekit/pull/2) (Merged 2026-10-03, commit `9b8989e`, feature commit `999b431`)
- **Evidence:**
  - Nested condition tree (`all`, `any`, `not`) aligned with `json-rules-engine`.
  - Event objects (`events[]` with `id`, `type`, `params`).
  - Proposal parameter schema validation: Draft-07 JSON Schema validation at proposal time (`propose(rule, registry)`).
  - Ingestion compatibility: Legacy v0.1 JSON with `when`/`then` normalizes to v2 in-memory.
  - JSON Schema & Fixtures: Added `schema/rule.schema.json`, `schema/fixtures/v1_legacy_rule.json`, `schema/fixtures/v2_basic_rule.json`, `schema/fixtures/v2_nested_conditions.json`.
  - Architecture: Documented in `docs/ADR/0002-industry-standard-rule-documents.md`.
  - Test suites: Verified passing across Rust (`tests/engine_tests.rs`, `tests/schema_golden.rs`) and Dart (`test/engine_test.dart`).

---

## 2. Release & Verification Evidence Baseline

### [x] Rust Engine & Package Verification (`crates/dc_rulekit`)
- **Status:** VERIFIED & PUBLISHED
- **Evidence:**
  - `cargo test --all`: 15 tests passed (11 engine tests, 2 schema golden tests, 2 demo plugin tests).
  - `cargo package -p dc_rulekit --offline`: 18 files packaged (58.7 KiB), successfully verified and compiled.
  - Registry: Version `0.2.0` published on crates.io on October 3, 2026 by `ucguy4u`.
  - Consumer smoke test: Clean external project `/tmp/rulekit_crates_io_smoke` resolved and compiled `dc_rulekit = "0.2.0"` directly from crates.io.

### [x] Dart Mirror Package Verification (`packages/dc_rulekit`)
- **Status:** VERIFIED & PACKAGED (Pending pub.dev credentials)
- **Evidence:**
  - `dart analyze`: 0 issues found across `packages/dc_rulekit`.
  - `dart test`: 8 tests passed (nested conditions, events, fail-closed plugins, proposal lifecycle, audit hook, v1 compat, schema validation).
  - `dart pub publish --dry-run`: 0 warnings; 8 KB archive validated.
  - Consumer smoke test: Standalone consumer project executed legacy v1 normalization and engine evaluation successfully.

### [x] Downstream Consumer Compatibility Analysis
- **Status:** MAPPED & VERIFIED
- **Evidence:**
  - Consumer identified: `cruftkit-model-fetch/packages/rulekit_adapter`.
  - Consumer dependency: Pinned to `dc_rulekit: ^0.1.0`.
  - Isolation confirmed: SemVer guarantees `^0.1.0` consumers are not broken by `0.2.0`.
  - Migration plan: Full API and field mapping documented in `RULEKIT_RELEASE_PLAN.md`.

---

## 3. Active Release Tasks

### [ ] Publish Dart Package to pub.dev *(Pending — maintainer OAuth)*
- **Task:** Execute `dart pub publish` in `packages/dc_rulekit`.
- **Pre-requisite:** Requires interactive login / OAuth token with publishing access for `dc_rulekit` on pub.dev (authorized publisher account).
- **Agent note:** Cannot be completed in Cloud Agent VMs (no interactive pub.dev OAuth; run locally or in maintainer CI with secret).
- **Verification:** Confirm package live on `https://pub.dev/packages/dc_rulekit`.

### [ ] Update Downstream Cruftkit Adapter to v0.2 *(Pending — Cruftkit repo)*
- **Task:** Upgrade `packages/rulekit_adapter` in the Cruftkit workspace from `dc_rulekit: ^0.1.0` to `^0.2.0`.
- **Target Files:**
  - `packages/rulekit_adapter/pubspec.yaml`: Bump to `dc_rulekit: ^0.2.0`.
  - `packages/rulekit_adapter/lib/src/bridge.dart`: Update `when` to `conditions`, `then` to `events`, and pass `registry` into `propose(rule, registry)`.
  - `packages/rulekit_adapter/test/adapter_test.dart`: Update test fixtures and expectations.
- **Agent note:** Cruftkit is a separate repository; migration steps remain documented in `RULEKIT_RELEASE_PLAN.md` until that repo is updated.

---

## 4. Explicitly Deferred Work (Future Milestones)

- [ ] **Rust ↔ Dart FFI Integration**
  - *Status:* Deferred per ADR 0001.
  - *Next Action:* Author ADR evaluating `flutter_rust_bridge` vs `uniffi` when binary unification is required for mobile.
- [ ] **Cron / Trigger Execution Scheduler**
  - *Status:* Deferred per ADR 0001.
  - *Next Action:* Design an external trigger scheduler service or library that schedules and executes rule evaluations.
- [ ] **Cruftkit Host-Specific Adapter in Core**
  - *Status:* Out of scope per ADR 0001.
  - *Next Action:* Adapter remains inside the Cruftkit repository.
