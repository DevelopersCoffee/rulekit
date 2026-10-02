# rulekit

Business-agnostic on-device rules engine for DevelopersCoffee apps.

> **Published package name:** [`dc_rulekit`](https://crates.io/crates/dc_rulekit) (Rust) and [`dc_rulekit`](https://pub.dev/packages/dc_rulekit) (Dart).  
> **This repository** is named `rulekit` on GitHub — same project, different registry names (crates.io `rulekit` / `rule_kit` are taken).

Host applications register **condition** and **action** plugins; the core validates, stores, and evaluates rules without domain-specific effects.

## Concepts

- **Rule**: JSON document with `when` (conditions) and `then` (actions), optional `trigger`, `source` (`static` | `llm`).
- **Plugins**: String ids → host `ConditionEvaluator` / `ActionHandler`.
- **Lifecycle**: `propose` → `approve` → active rule in store.
- **Evaluate**: Ordered conditions (all must pass), ordered actions, optional **dry-run** and **audit receipt** hook.

See [docs/ADR/0001-agnostic-core-vs-host-plugins.md](docs/ADR/0001-agnostic-core-vs-host-plugins.md).

## Rust quickstart

```toml
[dependencies]
dc_rulekit = "0.1"
```

```rust
use dc_rulekit::{
    Action, Condition, Engine, EvalContext, EvaluateOptions, PluginRegistry,
    ProposalStore, Rule, RuleSource, RuleStore,
};
use dc_rulekit_demo_plugins::{register_all, PLUGIN_ALWAYS, PLUGIN_LOG};
use serde_json::json;

let mut registry = PluginRegistry::new();
register_all(&mut registry);
let engine = Engine::new(&registry);

let mut rule = Rule::new("demo.app/hello", "Hello", RuleSource::Static);
rule.when.push(Condition {
    id: "always".into(),
    plugin: PLUGIN_ALWAYS.into(),
    params: json!({}),
});
rule.then.push(Action {
    id: "log".into(),
    plugin: PLUGIN_LOG.into(),
    params: json!({ "message": "hello from dc_rulekit" }),
});

let mut proposals = ProposalStore::in_memory();
let mut active = RuleStore::in_memory();
let proposal = proposals.propose(rule).unwrap();
let active_rule = proposals.approve(&proposal.proposal_id, &mut active).unwrap();

let receipt = engine
    .evaluate(&active_rule, &EvalContext::new("demo.app"), EvaluateOptions::default())
    .unwrap();
assert!(receipt.matched);
```

Run tests:

```bash
cargo test
```

Run the demo quickstart (Rust):

```bash
cargo test -p dc_rulekit_demo_plugins quickstart_runs
```

## Dart quickstart

```yaml
dependencies:
  dc_rulekit: ^0.1.0
```

```dart
import 'package:dc_rulekit/dc_rulekit.dart';

final registry = PluginRegistry()
  ..registerCondition(/* host ConditionEvaluator */)
  ..registerAction(/* host ActionHandler */);

final engine = Engine(registry);
// Same propose → approve → evaluate flow as Rust.
```

```bash
cd packages/dc_rulekit && dart pub get && dart test
```

## Layout

| Path | Description |
|------|-------------|
| `crates/dc_rulekit` | Rust core (crates.io: `dc_rulekit`) |
| `crates/dc_rulekit_demo_plugins` | Toy plugins for examples/tests |
| `packages/dc_rulekit` | Dart package (pub.dev: `dc_rulekit`) |
| `docs/ADR/` | Architecture decisions |

## License

MIT — see [LICENSE](LICENSE).
