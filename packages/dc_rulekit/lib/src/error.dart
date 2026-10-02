sealed class RulekitException implements Exception {
  const RulekitException(this.message);
  final String message;
  @override
  String toString() => message;
}

class UnknownConditionPlugin extends RulekitException {
  UnknownConditionPlugin(this.pluginId) : super('unknown condition plugin: $pluginId');
  final String pluginId;
}

class UnknownActionPlugin extends RulekitException {
  UnknownActionPlugin(this.pluginId) : super('unknown action plugin: $pluginId');
  final String pluginId;
}

class DisabledRule extends RulekitException {
  DisabledRule(this.ruleId) : super('rule is disabled: $ruleId');
  final String ruleId;
}

class ConditionFailed extends RulekitException {
  ConditionFailed({
    required this.ruleId,
    required this.conditionId,
    required this.pluginId,
  }) : super(
          'condition failed: rule=$ruleId, condition=$conditionId, plugin=$pluginId',
        );
  final String ruleId;
  final String conditionId;
  final String pluginId;
}

class ActionDenied extends RulekitException {
  ActionDenied({
    required this.ruleId,
    required this.actionId,
    required this.pluginId,
    required this.reason,
  }) : super(
          'action denied: rule=$ruleId, action=$actionId, plugin=$pluginId, reason=$reason',
        );
  final String ruleId;
  final String actionId;
  final String pluginId;
  final String reason;
}

class SchemaVersionMismatch extends RulekitException {
  SchemaVersionMismatch({required this.expected, required this.found})
      : super('schema version mismatch: expected=$expected, found=$found');
  final int expected;
  final int found;
}

class RuleNotFound extends RulekitException {
  RuleNotFound(this.ruleId) : super('rule not found: $ruleId');
  final String ruleId;
}

class ProposalNotFound extends RulekitException {
  ProposalNotFound(this.proposalId) : super('proposal not found: $proposalId');
  final String proposalId;
}

class InvalidProposalState extends RulekitException {
  InvalidProposalState(this.detail) : super('invalid proposal state: $detail');
  final String detail;
}

class StoreError extends RulekitException {
  StoreError(this.detail) : super('store error: $detail');
  final String detail;
}
