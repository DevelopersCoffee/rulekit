import 'error.dart';

const currentSchemaVersion = 1;

enum RuleSource { static, llm }

enum TriggerType { manual, event, schedule }

class Trigger {
  const Trigger.manual() : type = TriggerType.manual, topic = null, expression = null;
  const Trigger.event({this.topic = ''})
      : type = TriggerType.event,
        expression = null;
  const Trigger.schedule({required this.expression})
      : type = TriggerType.schedule,
        topic = null;

  final TriggerType type;
  final String? topic;
  final String? expression;

  Map<String, dynamic> toJson() => switch (type) {
        TriggerType.manual => {'type': 'manual'},
        TriggerType.event => {'type': 'event', 'topic': topic ?? ''},
        TriggerType.schedule => {'type': 'schedule', 'expression': expression},
      };

  factory Trigger.fromJson(Map<String, dynamic> json) {
    final t = json['type'] as String? ?? 'manual';
    return switch (t) {
      'event' => Trigger.event(topic: json['topic'] as String? ?? ''),
      'schedule' => Trigger.schedule(expression: json['expression'] as String? ?? ''),
      _ => const Trigger.manual(),
    };
  }
}

class Condition {
  Condition({required this.id, required this.plugin, this.params = const {}});
  final String id;
  final String plugin;
  final Map<String, dynamic> params;

  Map<String, dynamic> toJson() => {
        'id': id,
        'plugin': plugin,
        'params': params,
      };

  factory Condition.fromJson(Map<String, dynamic> json) => Condition(
        id: json['id'] as String,
        plugin: json['plugin'] as String,
        params: Map<String, dynamic>.from(json['params'] as Map? ?? {}),
      );
}

class Action {
  Action({required this.id, required this.plugin, this.params = const {}});
  final String id;
  final String plugin;
  final Map<String, dynamic> params;

  Map<String, dynamic> toJson() => {
        'id': id,
        'plugin': plugin,
        'params': params,
      };

  factory Action.fromJson(Map<String, dynamic> json) => Action(
        id: json['id'] as String,
        plugin: json['plugin'] as String,
        params: Map<String, dynamic>.from(json['params'] as Map? ?? {}),
      );
}

class Rule {
  Rule({
    required this.id,
    required this.title,
    required this.source,
    this.schemaVersion = currentSchemaVersion,
    this.enabled = true,
    this.trigger = const Trigger.manual(),
    this.when = const [],
    this.then = const [],
  });

  final int schemaVersion;
  final String id;
  final String title;
  final RuleSource source;
  final bool enabled;
  final Trigger trigger;
  final List<Condition> when;
  final List<Action> then;

  void validateSchema() {
    if (schemaVersion != currentSchemaVersion) {
      throw SchemaVersionMismatch(
        expected: currentSchemaVersion,
        found: schemaVersion,
      );
    }
  }

  Map<String, dynamic> toJson() => {
        'schema_version': schemaVersion,
        'id': id,
        'title': title,
        'source': source.name,
        'enabled': enabled,
        'trigger': trigger.toJson(),
        'when': when.map((c) => c.toJson()).toList(),
        'then': then.map((a) => a.toJson()).toList(),
      };

  factory Rule.fromJson(Map<String, dynamic> json) => Rule(
        schemaVersion: json['schema_version'] as int? ?? currentSchemaVersion,
        id: json['id'] as String,
        title: json['title'] as String,
        source: RuleSource.values.byName(json['source'] as String? ?? 'static'),
        enabled: json['enabled'] as bool? ?? true,
        trigger: Trigger.fromJson(
          Map<String, dynamic>.from(json['trigger'] as Map? ?? {'type': 'manual'}),
        ),
        when: (json['when'] as List? ?? [])
            .map((e) => Condition.fromJson(Map<String, dynamic>.from(e as Map)))
            .toList(),
        then: (json['then'] as List? ?? [])
            .map((e) => Action.fromJson(Map<String, dynamic>.from(e as Map)))
            .toList(),
      );
}

enum ProposalStatus { proposed, approved, rejected }

class RuleProposal {
  RuleProposal({
    required this.proposalId,
    required this.status,
    required this.proposedAt,
    required this.rule,
  });

  final String proposalId;
  final ProposalStatus status;
  final DateTime proposedAt;
  final Rule rule;

  Map<String, dynamic> toJson() => {
        'proposal_id': proposalId,
        'status': status.name,
        'proposed_at': proposedAt.toUtc().toIso8601String(),
        'rule': rule.toJson(),
      };

  factory RuleProposal.fromJson(Map<String, dynamic> json) => RuleProposal(
        proposalId: json['proposal_id'] as String,
        status: ProposalStatus.values.byName(json['status'] as String),
        proposedAt: DateTime.parse(json['proposed_at'] as String),
        rule: Rule.fromJson(Map<String, dynamic>.from(json['rule'] as Map)),
      );
}

class ConditionOutcome {
  ConditionOutcome({
    required this.conditionId,
    required this.plugin,
    required this.passed,
    this.detail,
  });
  final String conditionId;
  final String plugin;
  final bool passed;
  final Map<String, dynamic>? detail;

  Map<String, dynamic> toJson() => {
        'condition_id': conditionId,
        'plugin': plugin,
        'passed': passed,
        if (detail != null) 'detail': detail,
      };
}

class ActionOutcome {
  ActionOutcome({
    required this.actionId,
    required this.plugin,
    required this.executed,
    required this.skippedDryRun,
    this.result,
  });
  final String actionId;
  final String plugin;
  final bool executed;
  final bool skippedDryRun;
  final Map<String, dynamic>? result;

  Map<String, dynamic> toJson() => {
        'action_id': actionId,
        'plugin': plugin,
        'executed': executed,
        'skipped_dry_run': skippedDryRun,
        if (result != null) 'result': result,
      };
}

class AuditReceipt {
  AuditReceipt({
    required this.ruleId,
    required this.matched,
    required this.dryRun,
    required this.conditionResults,
    required this.actionOutcomes,
    this.opaque = const {},
  });
  final String ruleId;
  final bool matched;
  final bool dryRun;
  final List<ConditionOutcome> conditionResults;
  final List<ActionOutcome> actionOutcomes;
  final Map<String, dynamic> opaque;

  Map<String, dynamic> toJson() => {
        'rule_id': ruleId,
        'matched': matched,
        'dry_run': dryRun,
        'condition_results': conditionResults.map((c) => c.toJson()).toList(),
        'action_outcomes': actionOutcomes.map((a) => a.toJson()).toList(),
        'opaque': opaque,
      };
}
