import 'audit.dart';
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
    var allPassed = true;

    for (final condition in rule.when) {
      final evaluator = registry.getCondition(condition.plugin);
      final passed = evaluator.evaluate(condition.params, ctx);
      conditionResults.add(ConditionOutcome(
        conditionId: condition.id,
        plugin: condition.plugin,
        passed: passed,
      ));
      if (!passed) {
        allPassed = false;
        break;
      }
    }

    final actionOutcomes = <ActionOutcome>[];
    if (allPassed) {
      for (final action in rule.then) {
        final handler = registry.getAction(action.plugin);
        if (options.dryRun && !handler.isPure) {
          actionOutcomes.add(ActionOutcome(
            actionId: action.id,
            plugin: action.plugin,
            executed: false,
            skippedDryRun: true,
          ));
          continue;
        }
        final result = handler.execute(action.params, ctx);
        actionOutcomes.add(ActionOutcome(
          actionId: action.id,
          plugin: action.plugin,
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
