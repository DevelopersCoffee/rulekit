use crate::error::{Result, RulekitError};
use serde_json::Value;

/// Minimal draft-07 subset: object type, required keys, property types, additionalProperties.
pub fn validate_params_against_schema(
    plugin_id: &str,
    params: &Value,
    schema: &Value,
) -> Result<()> {
    let Some(obj) = params.as_object() else {
        return Err(RulekitError::InvalidPluginParams {
            plugin_id: plugin_id.to_string(),
            message: "params must be a JSON object".into(),
        });
    };

    if schema.get("type").and_then(|v| v.as_str()) == Some("object") {
        if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
            let allowed: std::collections::HashSet<_> = schema
                .get("properties")
                .and_then(|v| v.as_object())
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            for key in obj.keys() {
                if !allowed.contains(key) {
                    return Err(RulekitError::InvalidPluginParams {
                        plugin_id: plugin_id.to_string(),
                        message: format!("additional property not allowed: {key}"),
                    });
                }
            }
        }
        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            for req in required {
                let Some(key) = req.as_str() else { continue };
                if !obj.contains_key(key) {
                    return Err(RulekitError::InvalidPluginParams {
                        plugin_id: plugin_id.to_string(),
                        message: format!("missing required property: {key}"),
                    });
                }
            }
        }
        if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
            for (key, value) in obj {
                if let Some(prop_schema) = props.get(key) {
                    validate_value(plugin_id, key, value, prop_schema)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_value(plugin_id: &str, path: &str, value: &Value, schema: &Value) -> Result<()> {
    match schema.get("type").and_then(|v| v.as_str()) {
        Some("string") if !value.is_string() => Err(RulekitError::InvalidPluginParams {
            plugin_id: plugin_id.to_string(),
            message: format!("{path} must be string"),
        }),
        Some("number") if !value.is_number() => Err(RulekitError::InvalidPluginParams {
            plugin_id: plugin_id.to_string(),
            message: format!("{path} must be number"),
        }),
        Some("boolean") if value.as_bool().is_none() => Err(RulekitError::InvalidPluginParams {
            plugin_id: plugin_id.to_string(),
            message: format!("{path} must be boolean"),
        }),
        _ => Ok(()),
    }
}
