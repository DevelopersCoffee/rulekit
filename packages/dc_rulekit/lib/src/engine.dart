import 'audit.dart';
import 'conditions.dart';
import 'context.dart';
import 'error.dart';
import 'model.dart';
import 'plugin.dart';

class EvaluateOptions {
  const EvaluateOptions({this.dryRun = false});
  final bool dryRun;
}

class Engine {
  Engine(this.registry);
  final PluginRegistry registry;

  AuditReceipt evaluate(Rule rule, EvalContext ctx, [EvaluateOptions options = const EvaluateOptions()]) {
    rule.validateSchema();
    if (!rule.enabled) throw DisabledRule(rule.id);
    registry.validateRulePlugins(rule);

    final conditionResults = <ConditionOutcome>[];
    final allPassed = _evaluateConditions(rule.conditions, ctx, conditionResults, failFast: true);

    final actionOutcomes = <ActionOutcome>[];
    if (allPassed) {
      for (final action in rule.events) {
        final handler = registry.getAction(action.pluginId);
        if (options.dryRun && !handler.isPure) {
          actionOutcomes.add(ActionOutcome(
            actionId: action.id,
            eventType: action.eventType,
            executed: false,
            skippedDryRun: true,
          ));
          continue;
        }
        final result = handler.execute(action.params, ctx);
        actionOutcomes.add(ActionOutcome(
          actionId: action.id,
          eventType: action.eventType,
          executed: true,
          skippedDryRun: false,
          result: result,
        ));
      }
    }

    return AuditReceipt(
      ruleId: rule.id,
      matched: allPassed,
      dryRun: options.dryRun,
      conditionResults: conditionResults,
      actionOutcomes: actionOutcomes,
    );
  }

  bool _evaluateConditions(
    ConditionNode node,
    EvalContext ctx,
    List<ConditionOutcome> outcomes, {
    required bool failFast,
  }) {
    switch (node) {
      case AllConditionNode(:final all):
        if (all.isEmpty) return true;
        for (final child in all) {
          if (!_evaluateConditions(child, ctx, outcomes, failFast: failFast)) {
            return false;
          }
        }
        return true;
      case AnyConditionNode(:final any):
        if (any.isEmpty) return false;
        for (final child in any) {
          final branch = <ConditionOutcome>[];
          if (_evaluateConditions(child, ctx, branch, failFast: false)) {
            outcomes.addAll(branch);
            return true;
          }
          outcomes.addAll(branch);
        }
        return false;
      case NotConditionNode(:final not):
        return !_evaluateConditions(not, ctx, outcomes, failFast: false);
      case LeafConditionNode(:final condition):
        final evaluator = registry.getCondition(condition.plugin);
        final passed = evaluator.evaluate(condition.params, ctx);
        outcomes.add(ConditionOutcome(
          conditionId: condition.id,
          plugin: condition.plugin,
          passed: passed,
        ));
        return passed;
    }
  }

  AuditReceipt evaluateWithAudit(
    Rule rule,
    EvalContext ctx,
    EvaluateOptions options,
    AuditHook hook,
    Map<String, dynamic> opaque,
  ) {
    final receipt = evaluate(rule, ctx, options);
    final withOpaque = AuditReceipt(
      ruleId: receipt.ruleId,
      matched: receipt.matched,
      dryRun: receipt.dryRun,
      conditionResults: receipt.conditionResults,
      actionOutcomes: receipt.actionOutcomes,
      opaque: opaque,
    );
    hook.onReceipt(withOpaque);
    return withOpaque;
  }
}
