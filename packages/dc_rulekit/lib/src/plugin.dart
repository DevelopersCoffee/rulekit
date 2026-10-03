import 'error.dart';
import 'model.dart';
import 'context.dart';
import 'schema_validate.dart';

abstract class ConditionEvaluator {
  String get pluginId;
  bool evaluate(Map<String, dynamic> params, EvalContext ctx);
}

abstract class ActionHandler {
  String get pluginId;
  bool get isPure;
  Map<String, dynamic> execute(Map<String, dynamic> params, EvalContext ctx);
}

/// Optional JSON Schema for plugin params (implement on evaluators/handlers that need propose-time validation).
abstract interface class ParamsSchemaProvider {
  Map<String, dynamic>? get paramsSchema;
}

Map<String, dynamic>? paramsSchemaFor(Object plugin) {
  if (plugin is ParamsSchemaProvider) return plugin.paramsSchema;
  return null;
}

class PluginRegistry {
  final _conditions = <String, ConditionEvaluator>{};
  final _actions = <String, ActionHandler>{};

  void registerCondition(ConditionEvaluator evaluator) {
    _conditions[evaluator.pluginId] = evaluator;
  }

  void registerAction(ActionHandler handler) {
    _actions[handler.pluginId] = handler;
  }

  ConditionEvaluator getCondition(String pluginId) {
    final e = _conditions[pluginId];
    if (e == null) throw UnknownConditionPlugin(pluginId);
    return e;
  }

  ActionHandler getAction(String pluginId) {
    final h = _actions[pluginId];
    if (h == null) throw UnknownActionPlugin(pluginId);
    return h;
  }

  void validateRulePlugins(Rule rule) {
    for (final c in rule.conditions.leaves()) {
      getCondition(c.plugin);
    }
    for (final a in rule.events) {
      getAction(a.pluginId);
    }
  }

  void validateRuleParams(Rule rule) {
    for (final c in rule.conditions.leaves()) {
      final evaluator = getCondition(c.plugin);
      final schema = paramsSchemaFor(evaluator);
      if (schema != null) {
        validateParamsAgainstSchema(c.plugin, c.params, schema);
      }
    }
    for (final a in rule.events) {
      final handler = getAction(a.pluginId);
      final schema = paramsSchemaFor(handler);
      if (schema != null) {
        validateParamsAgainstSchema(a.pluginId, a.params, schema);
      }
    }
  }
}
