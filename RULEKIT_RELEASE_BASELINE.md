# Rulekit Release Baseline (v0.2.0)

**Date of Record:** October 16, 2026  
**Repository:** [DevelopersCoffee/rulekit](https://github.com/DevelopersCoffee/rulekit)  
**Target Release Version:** `0.2.0`  
**Git tip on `main`:** `9d7ad5b1ce378e20f7c6bc8d2bdf3a703c514810` (merge of [PR #4](https://github.com/DevelopersCoffee/rulekit/pull/4); docs-only drift on top of v0.2.0 code)  
**Release verification baseline (v0.2.0 code):** `9b8989e56ff92a3b89af77de256410adbd684231` (merge of [PR #2](https://github.com/DevelopersCoffee/rulekit/pull/2))  

---

## 1. Executive Summary

This document establishes the official verification and release baseline for `dc_rulekit` version `0.2.0`. Pull Request #1 (initial v0.1 engine) and Pull Request #2 (v0.2 industry-standard rule documents, nested conditions, event objects, and legacy v0.1 ingest normalization) are fully merged into `main`.

A rigorous local and registry baseline has been conducted across both Rust and Dart implementations:
- **Rust core (`dc_rulekit`)**: `0.2.0` is **already published on crates.io** (published October 3, 2026 by `ucguy4u`). Local tests, packaging, and an external clean consumer test fetching `dc_rulekit = "0.2.0"` from crates.io passed completely.
- **Dart mirror (`dc_rulekit`)**: `0.2.0` is packaged in `packages/dc_rulekit`. Local static analysis (`dart analyze`) reports 0 issues and unit/golden tests (`dart test`) pass 100%. `dart pub publish --dry-run` succeeded with 0 warnings. The package is ready for publication to `pub.dev` pending authorized account credentials.
- **Downstream consumer (`Cruftkit`)**: The downstream consumer `rulekit_adapter` in Cruftkit is pinned to `^0.1.0`. The upgrade path and API changes (`propose(rule, registry)`, `conditions`, `events`) are documented and verified.

---

## 2. Git & Commit Inventory

### 2.1 Commit Lineage on `main`

| Commit SHA | Author | Date | Subject & Scope | PR Reference |
| :--- | :--- | :--- | :--- | :--- |
| `9d7ad5b` | Uday Chauhan | 2026-11-22 | Merge pull request #4 from DevelopersCoffee/cursor/docs-drift-main-tip-2031 | [PR #4](https://github.com/DevelopersCoffee/rulekit/pull/4) |
| `8a2d7f2` | Cursor Agent | 2026-11-22 | docs: point release baseline at main after PR #3 merge | Feature commit for [PR #4](https://github.com/DevelopersCoffee/rulekit/pull/4) |
| `7e6de7f` | Uday Chauhan | 2026-11-17 | Merge pull request #3 from DevelopersCoffee/cursor/docs-drift-weekly-9735 | [PR #3](https://github.com/DevelopersCoffee/rulekit/pull/3) |
| `cb44513` | Cursor Agent | 2026-11-16 | docs: fix PR #2 merge SHA and baseline branch inventory | Follow-up on [PR #3](https://github.com/DevelopersCoffee/rulekit/pull/3) |
| `2d55a4e` | Cursor Agent | 2026-11-16 | docs: realign ADR 0001 and release baseline with v0.2 main | Feature commit for [PR #3](https://github.com/DevelopersCoffee/rulekit/pull/3) |
| `1c3aac0` | uday chauhan | 2026-10-16 | docs: add release baseline, plan, backlog, and todos for v0.2.0 | — |
| `9b8989e` | Uday Chauhan | 2026-10-03 | Merge pull request #2 from DevelopersCoffee/cursor/industry-standard-rules-1f87 | [PR #2](https://github.com/DevelopersCoffee/rulekit/pull/2) |
| `999b431` | Cursor Agent & Uday Chauhan | 2026-10-02 | feat(dc_rulekit): v0.2 industry-standard rule documents | Feature commit for PR #2 |
| `fe4654c` | Pub.dev (`coffee.devloper@gmail.com`) | 2026-10-02 | chore: publish metadata for dc_rulekit (LICENSE/README/CHANGELOG + crate repo URL) | Release metadata prep |
| `794d5a9` | Uday Chauhan | 2026-10-02 | Merge pull request #1 from DevelopersCoffee/cursor/rulekit-v0-1-d401 | [PR #1](https://github.com/DevelopersCoffee/rulekit/pull/1) |
| `5f3bf6d` | Cursor Agent & Uday Chauhan | 2026-10-02 | feat: add dc_rulekit v0.1 rules engine (Rust + Dart) | Feature commit for PR #1 |
| `a8cfcda` | Uday Chauhan | 2026-10-02 | chore: initial README | Repository initialization |

### 2.2 Branches & Worktrees Status

- **Local Branches:** `main` (clean working tree, tracking `origin/main`).
- **Remote Branches:**
  - `origin/main`: matches local `main` at `9d7ad5b` (release code unchanged since `9b8989e`; tip adds project docs only).
- **Working Tree:** Clean, no uncommitted changes or unstaged files.

---

## 3. Package & Manifest Inventory

### 3.1 Naming Distinction
- **GitHub Repository:** `DevelopersCoffee/rulekit` (crates.io names `rulekit` and `rule_kit` are taken by third parties).
- **Published Rust Crate:** `dc_rulekit`
- **Published Dart Package:** `dc_rulekit`

### 3.2 Rust Crate (`crates/dc_rulekit`)
- **Package Manifest:** `crates/dc_rulekit/Cargo.toml`
- **Declared Version:** `0.2.0`
- **Edition:** `2021`
- **License:** `MIT`
- **Repository:** `https://github.com/DevelopersCoffee/rulekit`
- **Homepage:** `https://github.com/DevelopersCoffee/rulekit`
- **Description:** Business-agnostic on-device rules engine with host-supplied plugins
- **Keywords:** `["rules", "engine", "plugins"]`
- **Categories:** `["config"]`
- **Dependencies:**
  - `serde` 1 (features: derive)
  - `serde_json` 1
  - `thiserror` 2
  - `uuid` =1.11.0 (features: v4, serde)
  - `chrono` 0.4 (features: serde)
- **Dev-Dependencies:** `tempfile` 3

### 3.3 Demo Plugins Crate (`crates/dc_rulekit_demo_plugins`)
- **Package Manifest:** `crates/dc_rulekit_demo_plugins/Cargo.toml`
- **Declared Version:** `0.1.0`
- **Publish Status:** Internal toy plugins for tests and examples (`publish = false` or example crate).
- **Dependency:** `dc_rulekit = { path = "../dc_rulekit" }`

### 3.4 Dart Package (`packages/dc_rulekit`)
- **Package Manifest:** `packages/dc_rulekit/pubspec.yaml`
- **Declared Version:** `0.2.0`
- **SDK Constraint:** `^3.5.0`
- **License:** `MIT` (`packages/dc_rulekit/LICENSE`)
- **Repository:** `https://github.com/DevelopersCoffee/rulekit`
- **Description:** Business-agnostic on-device rules engine (Dart mirror of the Rust dc_rulekit core).
- **Dev-Dependencies:** `lints: ^5.0.0`, `test: ^1.25.0`

---

## 4. Registry Status & Evidence

### 4.1 crates.io (Rust)
- **Package Page:** `https://crates.io/crates/dc_rulekit`
- **Published Version:** `0.2.0`
- **Publication Date:** October 3, 2026
- **Owner Account:** `ucguy4u`
- **Verification Proof:**
  - Created a clean temporary project outside the repository (`/tmp/rulekit_crates_io_smoke`).
  - Added dependency: `cargo add dc_rulekit@0.2.0`.
  - Resolution output: `Downloaded dc_rulekit v0.2.0`, `Checking dc_rulekit v0.2.0`, build completed successfully.
- **Status:** **PUBLISHED & VERIFIED**

### 4.2 pub.dev (Dart)
- **Package Page:** `https://pub.dev/packages/dc_rulekit`
- **Published Version:** None (Package not yet published on pub.dev).
- **Pre-publish Validation:**
  - Command: `dart pub publish --dry-run`
  - Output:
    ```text
    Publishing dc_rulekit 0.2.0 to https://pub.dev:
    ├── CHANGELOG.md (<1 KB)
    ├── LICENSE (1 KB)
    ├── README.md (<1 KB)
    ├── analysis_options.yaml (<1 KB)
    ├── lib
    │   ├── dc_rulekit.dart (<1 KB)
    │   └── src
    │       ├── audit.dart (<1 KB)
    │       ├── conditions.dart (4 KB)
    │       ├── context.dart (<1 KB)
    │       ├── engine.dart (3 KB)
    │       ├── error.dart (2 KB)
    │       ├── model.dart (6 KB)
    │       ├── plugin.dart (2 KB)
    │       ├── proposal.dart (2 KB)
    │       ├── schema_validate.dart (1 KB)
    │       └── store.dart (1 KB)
    ├── pubspec.yaml (<1 KB)
    └── test
        └── engine_test.dart (6 KB)

    Total compressed archive size: 8 KB.
    Validating package...
    Package has 0 warnings.
    ```
- **Status:** **PACKAGED & VALIDATED (Pending authorized publication)**

---

## 5. Verification Test Suite Baseline

### 5.1 Rust Test Suite
- **Command:** `cargo test --all`
- **Result:** 15 tests passed; 0 failed; 0 ignored; 0 filtered out.

| Test Binary | Test Name | Result | Feature Verified |
| :--- | :--- | :--- | :--- |
| `tests/engine_tests.rs` | `disabled_rule_errors` | `ok` | Disabled rules fail closed |
| `tests/engine_tests.rs` | `proposal_approve_and_store_roundtrip` | `ok` | Proposal lifecycle & store roundtrip |
| `tests/engine_tests.rs` | `dry_run_skips_impure_actions` | `ok` | Dry-run skips impure actions |
| `tests/engine_tests.rs` | `evaluate_runs_actions_when_conditions_pass` | `ok` | Engine evaluates conditions & triggers actions |
| `tests/engine_tests.rs` | `audit_hook_receives_opaque_payload` | `ok` | Audit receipts with opaque facts |
| `tests/engine_tests.rs` | `dry_run_runs_pure_actions` | `ok` | Dry-run executes pure actions |
| `tests/engine_tests.rs` | `nested_any_condition` | `ok` | Nested `any`/`all`/`not` boolean logic |
| `tests/engine_tests.rs` | `propose_rejects_invalid_params_schema` | `ok` | Parameter schema validation at propose time |
| `tests/engine_tests.rs` | `unknown_plugin_fail_closed` | `ok` | Fail-closed on missing plugins |
| `tests/engine_tests.rs` | `v1_rule_json_compat_deserializes_and_runs` | `ok` | Legacy v0.1 JSON ingestion to v2 in-memory |
| `tests/engine_tests.rs` | `store_file_roundtrip` | `ok` | Durable file store read/write roundtrip |
| `tests/schema_golden.rs` | `v1_fixture_upgrades_to_v2_rule` | `ok` | `schema/fixtures/v1_legacy_rule.json` normalization |
| `tests/schema_golden.rs` | `golden_v2_fixtures_deserialize_and_validate` | `ok` | `schema/fixtures/v2_*.json` validate against `rule.schema.json` |
| `dc_rulekit_demo_plugins` | `tests::threshold_condition` | `ok` | Demo plugin threshold evaluation |
| `dc_rulekit_demo_plugins` | `tests::quickstart_runs` | `ok` | End-to-end quickstart execution |

### 5.2 Rust Packaging Verification
- **Command:** `cargo package -p dc_rulekit --offline`
- **Result:** `Packaged 18 files, 58.7KiB (14.1KiB compressed)`.
- **Verification:** Verified package compilation and test run against unpacked archive.

### 5.3 Dart Static Analysis & Test Suite
- **Static Analysis Command:** `dart analyze` in `packages/dc_rulekit`
- **Result:** `No issues found!` (0 warnings, 0 errors, 0 lints).
- **Test Command:** `dart test` in `packages/dc_rulekit`
- **Result:** 8 tests passed; 0 failed.

| Test Case | Result | Verified Capability |
| :--- | :--- | :--- |
| `evaluate runs actions when conditions pass` | `ok` | Core engine evaluation |
| `dry-run skips impure actions` | `ok` | Action purity and dry-run flag |
| `unknown plugin fail-closed` | `ok` | UnknownConditionPlugin & UnknownActionPlugin |
| `proposal approve lifecycle` | `ok` | `propose(rule, registry)` → `approve` → `RuleStore` |
| `audit hook receives opaque payload` | `ok` | Receipt emission with action outcomes |
| `v1 json compat` | `ok` | Deserializing v1 `when`/`then` maps into v2 `conditions`/`events` |
| `golden v2 fixture roundtrip` | `ok` | Ingestion of `v2_nested_conditions.json` |
| `propose validates params schema` | `ok` | Draft-07 schema subset parameter validation |

---

## 6. Architecture & ADR Alignment

- **ADR 0001 (Agnostic Core vs Host Plugins):**
  - Confirmed: The core engine has no domain-specific knowledge (no storage cleanup, file manipulation, or billing).
  - Confirmed: Host applications supply plugins via `PluginRegistry`.
  - Confirmed: FFI and Cron/Scheduler remain explicitly deferred.
- **ADR 0002 (Industry-Standard Rule Documents):**
  - Confirmed: Rule schema v2 implements `conditions` tree (`all`, `any`, `not`) and `events` array (`id`, `type`, `params`).
  - Confirmed: v0.1 legacy JSON normalization ensures non-breaking reads for legacy stores.
  - Confirmed: Proposal API signature updated to `propose(rule, &registry)` / `propose(rule, registry)` for propose-time schema validation.

---

## 7. Baseline Conclusion

The v0.2 implementation at commit `9b8989e` remains functionally stable, internally consistent, and thoroughly tested across Rust and Dart implementations. `main` at `9d7ad5b` adds release-tracking and drift-alignment docs only (`RULEKIT_*`, `TODOS.md`, ADR 0001). crates.io already has the published `dc_rulekit 0.2.0` package. pub.dev is completely prepared for publication. All baseline criteria for Sprint 0 and Sprint 1 are satisfied.
