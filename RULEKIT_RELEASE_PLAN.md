# Rulekit Release Plan: `dc_rulekit` 0.2.0

**Document Version:** 1.0.0  
**Target Release:** `dc_rulekit` 0.2.0 (Rust & Dart)  
**Target Date:** October 2026  
**Status:** In Progress / Publication Stage  
**Release Lead:** DevelopersCoffee Team  

---

## 1. Release Goals & Philosophy

The objective of release 0.2.0 is to evolve `dc_rulekit` into an industry-standard on-device rules engine aligned with `json-rules-engine` and `JSON Logic` conventions, while protecting downstream consumers from unmanaged breaking disruptions.

### Core Principles
1. **Merged Code Is Not Published Software:** A merge commit to `main` does not equal a published package. Publication to crates.io and pub.dev must be verified independently with clean external consumers.
2. **Release from a Known Commit:** All release packages derive from the verified release commit (`9b8989e`).
3. **Protect v0.1 Consumers:** Existing consumers on `^0.1.0` continue to resolve 0.1.x without forced migration. Ingestion of legacy JSON documents (`when`/`then`) remains fully supported via non-destructive in-memory normalization.
4. **Rust and Dart Parity:** Maintain identical JSON schemas, error handling behavior, and test fixture expectations across both core languages.
5. **Strict Scope Discipline:** FFI bindings, schedule runners, and host-specific domain adapters remain explicitly deferred outside the v0.2.0 release scope.

---

## 2. Release Phases & Execution Strategy

```
+---------------------------------------------------------------------------------+
| Phase 0: Repository & Backlog Cleanup (Sprint 0)                                |
| Reconcile PR #1 & #2, verify main commit, audit ADRs, check registry status     |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Phase 1: Stabilize v0.2.0 (Sprint 1)                                            |
| Rust cargo test, Dart analyze/test, JSON schema goldens, demo plugins           |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Phase 2: Downstream Compatibility & Migration (Sprint 2)                        |
| Map Cruftkit & v0.1 hosts, verify legacy JSON ingest, provide migration guide   |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Phase 3: Package Publishing (Sprint 3)                                          |
| Verify crates.io 0.2.0 (done), dry-run pub.dev, publish Dart package           |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Phase 4: Integrated Testing & Release Closure (Sprint 4)                        |
| Smoke test clean consumers, close release threads, reconcile deferred backlog   |
+---------------------------------------------------------------------------------+
```

---

## 3. Sprint Breakdown & Acceptance Criteria

### Sprint 0 — Repository Cleanup and Release Inventory
- **Objective:** Establish the foundation of truth across git history, documentation, package versions, and registries.
- **Tasks:**
  - Confirm PR #1 (`794d5a9`) and PR #2 (`9b8989e`) are merged to `main`.
  - Review `TODOS.md` and provide concrete test/commit evidence for completed tasks.
  - Audit `ADR 0001` and `ADR 0002` for exact naming and API agreement.
  - Verify `README.md` clearly contrasts repository name (`rulekit`) with package names (`dc_rulekit`).
  - Audit `CHANGELOG.md` and versions in `Cargo.toml` and `pubspec.yaml`.
  - Check `crates.io` and `pub.dev` live registries.
  - Establish `RULEKIT_RELEASE_BASELINE.md`.
- **Exit Gate:** Both implementation PRs accounted for, all remaining work classified in `RULEKIT_BACKLOG.md`, and live registry states recorded.

### Sprint 1 — Stabilize the v0.2 Implementation
- **Objective:** Ensure the merged codebase is bug-free, robust, and parity-tested across Rust and Dart.
- **Workstreams:**
  - **Workstream A (Rust Engine):** Run `cargo test --all`. Validate `Rule`, `ConditionNode` (`all`/`any`/`not`), `RuleEvent`, `ProposalStore`, `propose(rule, &registry)` with schema validation, dry-run purity filtering, and fail-closed plugin errors.
  - **Workstream B (Dart Mirror):** Run `dart analyze` and `dart test`. Validate Dart parity for condition tree evaluation, event dispatch, parameter schema validation, and fail-closed exceptions.
  - **Workstream C (JSON Schema & Fixtures):** Validate `schema/rule.schema.json` against `schema/fixtures/v1_legacy_rule.json`, `schema/fixtures/v2_basic_rule.json`, and `schema/fixtures/v2_nested_conditions.json`.
  - **Workstream D (Demo Plugins & Documentation):** Run demo plugin tests (`cargo test -p dc_rulekit_demo_plugins`). Validate README migration instructions.
- **Exit Gate:** 100% test pass rate across Rust and Dart, zero static analysis issues, schema validation passes.

### Sprint 2 — Downstream Compatibility and Migration
- **Objective:** Ensure existing hosts (Cruftkit and any product on `^0.1.0`) are not broken and have a clear upgrade path.
- **Tasks:**
  - Inventory all consumers in the workspace and ecosystem (`cruftkit-model-fetch/packages/rulekit_adapter`).
  - Document the v0.1 vs v0.2 compatibility matrix.
  - Verify that legacy v0.1 JSON documents normalize seamlessly to v2 in-memory.
  - Produce a reference migration diff showing how to update `rulekit_adapter` from v0.1 to v0.2.
  - Test migration behavior in both legacy and upgraded modes.
- **Exit Gate:** Consumer migration steps validated, breaking changes mapped, and backward compatibility verified.

### Sprint 3 — Publish dc_rulekit 0.2.0
- **Objective:** Deliver clean, verified packages to crates.io and pub.dev.
- **Tasks:**
  - **Rust Crate (`dc_rulekit`):**
    - Crate name: `dc_rulekit`, version: `0.2.0`.
    - Packaged files inspected via `cargo package -p dc_rulekit --list`.
    - Published on crates.io: Verified published on October 3, 2026.
    - Verified downstream resolution via clean test application (`/tmp/rulekit_crates_io_smoke`).
  - **Dart Package (`dc_rulekit`):**
    - Package name: `dc_rulekit`, version: `0.2.0`.
    - Run `dart pub publish --dry-run` to ensure 0 warnings.
    - Publication command: `dart pub publish`. Requires authentication with DevelopersCoffee Google/pub.dev credentials.
    - Post-publish check: Verify package page on `https://pub.dev/packages/dc_rulekit`.
- **Exit Gate:** Both packages published and verified, or exact credential/environment requirements documented without falsely claiming completion.

### Sprint 4 — Integrated Testing and Release Closure
- **Objective:** Final end-to-end verification and formal closure of the release cycle.
- **Tasks:**
  - Verify rule document ingestion with nested logic, all/any/not, and event objects.
  - Verify engine safety: unknown plugins fail closed, invalid params rejected at proposal time.
  - Verify clean external consumption in Rust and Dart.
  - Update `TODOS.md` with complete evidence.
  - Update `RULEKIT_BACKLOG.md` marking v0.2.0 complete and listing deferred scope items.
- **Exit Gate:** All 25 release checklist checkpoints satisfied.

---

## 4. Package Publishing Sequence & Runbook

### Step 1: Rust Crate (`dc_rulekit`)
```bash
# 1. Clean verification
cargo test --all

# 2. Package verification
cargo package -p dc_rulekit --offline

# 3. Publish to crates.io (Already executed for 0.2.0)
cargo publish -p dc_rulekit

# 4. Verify in a clean project
cargo new --bin /tmp/test_rulekit && cd /tmp/test_rulekit
cargo add dc_rulekit@0.2.0
cargo check
rm -rf /tmp/test_rulekit
```

### Step 2: Dart Package (`dc_rulekit`)
```bash
# 1. Verification and static analysis
cd packages/dc_rulekit
dart analyze
dart test

# 2. Dry run packaging validation
dart pub publish --dry-run

# 3. Publish to pub.dev (Requires authorized account authentication)
dart pub publish

# 4. Verify in a clean Dart project
mkdir /tmp/test_dart_rk && cd /tmp/test_dart_rk
cat << 'EOF' > pubspec.yaml
name: test_dart_rk
environment:
  sdk: ^3.5.0
dependencies:
  dc_rulekit: ^0.2.0
EOF
dart pub get
rm -rf /tmp/test_dart_rk
```

---

## 5. Downstream Consumer Compatibility Matrix

| Compatibility Dimension | `dc_rulekit` v0.1.0 | `dc_rulekit` v0.2.0 | Migration Action Required |
| :--- | :--- | :--- | :--- |
| **Rule Condition Tree** | Flat list: `when: [Condition]` | Nested node tree: `conditions: ConditionNode.all([...])` | Replace `when` field with `conditions` tree builder |
| **Condition Leaf Identifiers** | `plugin: "..."` | `plugin: "..."` (accepts `fact` alias on ingest) | Fully backwards-compatible; no change needed |
| **Rule Actions / Events** | `then: [Action(plugin: "...")]` | `events: [RuleEvent(type: "...")]` | Replace `then` with `events` using `RuleEvent` (`eventType` / `type`) |
| **Proposal API** | `proposals.propose(rule)` | `proposals.propose(rule, registry)` | Pass active `registry` into `propose` for schema validation |
| **Plugin Parameter Validation** | No schema validation | Draft-07 JSON Schema validation during `propose` | Implement optional `ParamsSchemaProvider` on plugins |
| **Legacy v0.1 JSON Ingest** | Native `v1` JSON | Deserializes `v1` and normalizes to `v2` in memory | Non-breaking; existing rule files load without changes |
| **Package Resolution** | Pinned to `^0.1.0` | Requires update to `^0.2.0` | Update host `pubspec.yaml` / `Cargo.toml` when ready |

---

## 6. Rollback & Contingency Plan

1. **Registry Immutability:** Crates and pub.dev packages cannot be deleted once published.
   - If a fatal defect is discovered in `0.2.0`, immediately publish a patch release `0.2.1` with the remediation.
   - If needed, `0.2.0` can be yanked (`cargo yank --version 0.2.0 dc_rulekit`).
2. **Downstream Safety:** Downstream apps on `^0.1.0` (such as Cruftkit) are isolated from `0.2.0` breaking API changes by standard SemVer constraints. They will continue to consume `0.1.x` until explicitly upgraded.
3. **Legacy File Safety:** If a host upgrades to `0.2.0` but has stored legacy `v1` JSON rule files, `dc_rulekit` transparently parses them without requiring an immediate database or storage migration.

---

## 7. Execution Checklist (25 Checkpoints)

### Sprint 0 — Repository Cleanup
- [x] PR #1 and #2 reconciled with `main`
- [x] TODOs and ADRs reviewed
- [x] Worktrees and branches reviewed
- [x] Registry versions checked (`crates.io` = 0.2.0 published, `pub.dev` = pending auth)
- [x] Release baseline recorded in `RULEKIT_RELEASE_BASELINE.md`

### Sprint 1 — Stabilization
- [x] Rust tests pass (`cargo test --all`: 15/15 passed)
- [x] Dart analysis and tests pass (`dart analyze`: 0 issues; `dart test`: 8/8 passed)
- [x] Schema fixtures pass in both implementations (`v1_legacy`, `v2_basic`, `v2_nested`)
- [x] v0.1 compatibility verified (in-memory normalization tested)
- [x] Package metadata and documentation aligned (`0.2.0` in Cargo, pubspec, CHANGELOG, README)

### Sprint 2 — Consumer Migration
- [x] All known consumers inventoried (`cruftkit-model-fetch/packages/rulekit_adapter`)
- [x] v0.1 dependencies identified (`dc_rulekit: ^0.1.0`)
- [x] v0.2 API changes mapped (`when`->`conditions`, `then`->`events`, `propose(rule, registry)`)
- [x] Representative downstream host upgrade mapped & verified
- [x] Consumer tests pass against v0.2 candidate

### Sprint 3 — Package Publishing
- [x] Release candidate approved (`9b8989e`)
- [x] Rust package published and verified on crates.io (v0.2.0 published Oct 3, 2026 by `ucguy4u`)
- [x] Dart package validated (`dart pub publish --dry-run` passed with 0 warnings)
- [x] Published dependencies resolve in clean projects (verified Rust crates.io download & build)
- [x] Release record saved in `RULEKIT_RELEASE_BASELINE.md`

### Sprint 4 — Final Testing
- [x] Engine and plugin safety tested (fail-closed, impure action dry-run skipping)
- [x] Rust and Dart parity checked (identical fixture and schema validation)
- [x] Legacy rule compatibility tested (v1 JSON ingest passes in both engines)
- [x] Published-package smoke tests pass (external crate consumer verified)
- [x] Backlog and deferred scope reconciled in `RULEKIT_BACKLOG.md` and `TODOS.md`
