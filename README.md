# rulekit

Business-agnostic on-device rules engine for DevelopersCoffee apps.

> **Published package name:** [`dc_rulekit`](https://crates.io/crates/dc_rulekit) (Rust) and [`dc_rulekit`](https://pub.dev/packages/dc_rulekit) (Dart).  
> **This repository** is named `rulekit` on GitHub — same project, different registry names (crates.io `rulekit` / `rule_kit` are taken).

Host applications register **condition** and **action** plugins; the core validates, stores, and evaluates rules without domain-specific effects.

## Concepts

- **Rule (v2)**: JSON with nested `conditions` (`all` / `any` / `not`, json-rules-engine style) and `events` (`{ id, type, params }`).
- **Facts**: Plain JSON on [`EvalContext`](crates/dc_rulekit/src/context.rs) — no proprietary context shape.
- **Plugins**: String ids → host `ConditionEvaluator` / `ActionHandler`; optional JSON Schema on params (validated at **propose**).
- **Lifecycle**: `propose` → `approve` → active rule in store (unchanged from v0.1).
- **Evaluate**: Tree evaluation, ordered events, optional **dry-run** and **audit receipt** hook.

See [docs/ADR/0001-agnostic-core-vs-host-plugins.md](docs/ADR/0001-agnostic-core-vs-host-plugins.md) and [docs/ADR/0002-industry-standard-rule-documents.md](docs/ADR/0002-industry-standard-rule-documents.md).

## Rule JSON an LLM can emit (v0.2)

```json
{
  "schema_version": 2,
  "id": "my.app/high-value-alert",
  "title": "Alert when value crosses threshold",
  "source": "llm",
  "enabled": true,
  "trigger": { "type": "event", "topic": "facts.updated" },
  "conditions": {
    "all": [
      {
        "id": "check-value",
        "plugin": "my.when.threshold",
        "params": { "key": "order_total", "min": 100 }
      }
    ]
  },
  "events": [
    {
      "id": "notify",
      "type": "my.then.notify",
      "params": { "channel": "ops", "message": "High value order" }
    }
  ]
}
```

Nested logic (familiar from json-rules-engine):

```json
"conditions": {
  "any": [
    { "all": [
        { "id": "a", "plugin": "my.when.always", "params": {} },
        { "not": { "id": "b", "plugin": "my.when.maintenance", "params": {} } }
      ]
    }
  ]
}
```

**Migration from v0.1:** flat `when` / `then` with `plugin` still **parse** (upgraded to v2 in memory). New documents should use `schema_version: 2`, `conditions`, and `events[].type`. Cruftkit on `dc_rulekit ^0.1.0` is unaffected until it upgrades to `0.2`.

## Rust quickstart

```toml
[dependencies]
dc_rulekit = "0.2"
```

```rust
use dc_rulekit::{
    Condition, ConditionNode, Engine, EvalContext, EvaluateOptions, PluginRegistry,
    ProposalStore, Rule, RuleEvent, RuleSource, RuleStore,
};
use dc_rulekit_demo_plugins::{register_all, PLUGIN_ALWAYS, PLUGIN_LOG};
use serde_json::json;

let mut registry = PluginRegistry::new();
register_all(&mut registry);
let engine = Engine::new(&registry);

let mut rule = Rule::new("demo.app/hello", "Hello", RuleSource::Static);
rule.conditions = ConditionNode::all(vec![ConditionNode::leaf(Condition {
    id: "always".into(),
    plugin: PLUGIN_ALWAYS.into(),
    params: json!({}),
})]);
rule.events.push(RuleEvent {
    id: "log".into(),
    event_type: PLUGIN_LOG.into(),
    params: json!({ "message": "hello from dc_rulekit" }),
});

let mut proposals = ProposalStore::in_memory();
let mut active = RuleStore::in_memory();
let proposal = proposals.propose(rule, &registry)?;
let active_rule = proposals.approve(&proposal.proposal_id, &mut active)?;

let receipt = engine.evaluate(
    &active_rule,
    &EvalContext::new("demo.app"),
    EvaluateOptions::default(),
)?;
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
  dc_rulekit: ^0.2.0
```

```dart
import 'package:dc_rulekit/dc_rulekit.dart';

final registry = PluginRegistry()
  ..registerCondition(/* host ConditionEvaluator */)
  ..registerAction(/* host ActionHandler */);

final engine = Engine(registry);
// Same propose(rule, registry) → approve → evaluate flow as Rust.
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
| `schema/` | `rule.schema.json` + golden fixtures (CI) |
| `docs/ADR/` | Architecture decisions |

## License

MIT — see [LICENSE](LICENSE).
