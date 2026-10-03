import 'conditions.dart';
import 'error.dart';

const currentSchemaVersion = 2;

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

class Rule {
  Rule({
    required this.id,
    required this.title,
    required this.source,
    this.schemaVersion = currentSchemaVersion,
    this.enabled = true,
    this.trigger = const Trigger.manual(),
    ConditionNode? conditions,
    this.events = const [],
  }) : conditions = conditions ?? ConditionNode.all(const []);

  final int schemaVersion;
  final String id;
  final String title;
  final RuleSource source;
  final bool enabled;
  final Trigger trigger;
  final ConditionNode conditions;
  final List<RuleEvent> events;

  void validateSchema() {
    if (schemaVersion != currentSchemaVersion) {
      throw SchemaVersionMismatch(
        expected: currentSchemaVersion,
        found: schemaVersion,
      );
    }
    conditions.validateShape();
    for (final ev in events) {
      if (ev.id.isEmpty || ev.eventType.isEmpty) {
        throw EvaluationError('event requires non-empty id and type');
      }
    }
  }

  Map<String, dynamic> toJson() => {
        'schema_version': schemaVersion,
        'id': id,
        'title': title,
        'source': source.name,
        'enabled': enabled,
        'trigger': trigger.toJson(),
        'conditions': conditions.toJson(),
        'events': events.map((e) => e.toJson()).toList(),
      };

  factory Rule.fromJson(Map<String, dynamic> json) {
    final version = json['schema_version'] as int? ?? currentSchemaVersion;
    if (!readableSchemaVersions.contains(version)) {
      throw SchemaVersionMismatch(expected: currentSchemaVersion, found: version);
    }

    ConditionNode conditions;
    if (json['conditions'] != null) {
      conditions = ConditionNode.fromJson(json['conditions']);
    } else {
      final when = (json['when'] as List? ?? [])
          .map((e) => Condition.fromJson(Map<String, dynamic>.from(e as Map)))
          .toList();
      conditions = conditionsFromV1When(when);
    }

    List<RuleEvent> events;
    if ((json['events'] as List?)?.isNotEmpty ?? false) {
      events = (json['events'] as List)
          .map((e) => RuleEvent.fromJson(Map<String, dynamic>.from(e as Map)))
          .toList();
    } else {
      events = (json['then'] as List? ?? [])
          .map((e) => RuleEvent.fromJson(Map<String, dynamic>.from(e as Map)))
          .toList();
    }

    return Rule(
      schemaVersion: currentSchemaVersion,
      id: json['id'] as String,
      title: json['title'] as String,
      source: RuleSource.values.byName(json['source'] as String? ?? 'static'),
      enabled: json['enabled'] as bool? ?? true,
      trigger: Trigger.fromJson(
        Map<String, dynamic>.from(json['trigger'] as Map? ?? {'type': 'manual'}),
      ),
      conditions: conditions,
      events: events,
    );
  }
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
    required this.eventType,
    required this.executed,
    required this.skippedDryRun,
    this.result,
  });
  final String actionId;
  final String eventType;
  final bool executed;
  final bool skippedDryRun;
  final Map<String, dynamic>? result;

  Map<String, dynamic> toJson() => {
        'action_id': actionId,
        'type': eventType,
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
