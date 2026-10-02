import 'error.dart';
import 'model.dart';
import 'context.dart';

abstract class ConditionEvaluator {
  String get pluginId;
  bool evaluate(Map<String, dynamic> params, EvalContext ctx);
}

abstract class ActionHandler {
  String get pluginId;
  bool get isPure;
  Map<String, dynamic> execute(Map<String, dynamic> params, EvalContext ctx);
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
    for (final c in rule.when) {
      getCondition(c.plugin);
    }
    for (final a in rule.then) {
      getAction(a.plugin);
    }
  }
}
