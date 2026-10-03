use dc_rulekit::Rule;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
}

fn fixture(name: &str) -> Value {
    let path = repo_root().join("schema/fixtures").join(name);
    let text = fs::read_to_string(&path).expect("fixture readable");
    serde_json::from_str(&text).expect("valid fixture json")
}

#[test]
fn golden_v2_fixtures_deserialize_and_validate() {
    for name in ["v2_basic_rule.json", "v2_nested_conditions.json"] {
        let doc = fixture(name);
        assert_eq!(
            doc.get("schema_version").and_then(|v| v.as_u64()),
            Some(2),
            "{name}"
        );
        let rule: Rule = serde_json::from_value(doc).expect("fixture deserializes to Rule");
        rule.validate_schema().expect("rule validates");
    }
}

#[test]
fn v1_fixture_upgrades_to_v2_rule() {
    let doc = fixture("v1_legacy_rule.json");
    let rule: Rule = serde_json::from_value(doc).expect("v1 compat parse");
    assert_eq!(rule.schema_version, 2);
    assert!(!rule.conditions.leaves().is_empty());
    assert!(!rule.events.is_empty());
}
