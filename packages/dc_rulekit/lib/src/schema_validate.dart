import 'error.dart';

/// Minimal draft-07 subset: object type, required keys, property types, additionalProperties.
void validateParamsAgainstSchema(
  String pluginId,
  Map<String, dynamic> params,
  Map<String, dynamic> schema,
) {
  if (schema['type'] == 'object') {
    if (schema['additionalProperties'] == false) {
      final allowed = <String>{
        ...((schema['properties'] as Map?)?.keys.map((k) => k.toString()) ?? []),
      };
      for (final key in params.keys) {
        if (!allowed.contains(key)) {
          throw InvalidPluginParams(pluginId, 'additional property not allowed: $key');
        }
      }
    }
    for (final req in (schema['required'] as List? ?? [])) {
      if (!params.containsKey(req)) {
        throw InvalidPluginParams(pluginId, 'missing required property: $req');
      }
    }
    final props = schema['properties'] as Map? ?? {};
    for (final entry in params.entries) {
      final propSchema = props[entry.key];
      if (propSchema is Map) {
        _validateValue(pluginId, entry.key, entry.value, Map<String, dynamic>.from(propSchema));
      }
    }
  }
}

void _validateValue(
  String pluginId,
  String path,
  Object? value,
  Map<String, dynamic> schema,
) {
  final t = schema['type'];
  if (t == 'string' && value is! String) {
    throw InvalidPluginParams(pluginId, '$path must be string');
  }
  if (t == 'number' && value is! num) {
    throw InvalidPluginParams(pluginId, '$path must be number');
  }
  if (t == 'boolean' && value is! bool) {
    throw InvalidPluginParams(pluginId, '$path must be boolean');
  }
}
