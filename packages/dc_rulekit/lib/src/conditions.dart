import 'error.dart';

const readableSchemaVersions = [1, 2];

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
        plugin: (json['plugin'] ?? json['fact']) as String,
        params: Map<String, dynamic>.from(json['params'] as Map? ?? {}),
      );
}

sealed class ConditionNode {
  const ConditionNode();

  factory ConditionNode.all(List<ConditionNode> nodes) = AllConditionNode;
  factory ConditionNode.any(List<ConditionNode> nodes) = AnyConditionNode;
  factory ConditionNode.not(ConditionNode inner) = NotConditionNode;
  factory ConditionNode.leaf(Condition condition) = LeafConditionNode;

  List<Condition> leaves() {
    final out = <Condition>[];
    collectLeaves(out);
    return out;
  }

  void collectLeaves(List<Condition> out);

  void validateShape() {
    switch (this) {
      case AllConditionNode(:final all):
        for (final child in all) {
          child.validateShape();
        }
      case AnyConditionNode(:final any):
        for (final child in any) {
          child.validateShape();
        }
      case NotConditionNode(:final not):
        not.validateShape();
      case LeafConditionNode(:final condition):
        if (condition.id.isEmpty || condition.plugin.isEmpty) {
          throw EvaluationError('condition leaf requires non-empty id and plugin');
        }
    }
  }

  Map<String, dynamic> toJson() => switch (this) {
        AllConditionNode(:final all) => {
            'all': all.map((c) => c.toJson()).toList(),
          },
        AnyConditionNode(:final any) => {
            'any': any.map((c) => c.toJson()).toList(),
          },
        NotConditionNode(:final not) => {
            'not': not.toJson(),
          },
        LeafConditionNode(:final condition) => condition.toJson(),
      };

  static ConditionNode fromJson(dynamic json) {
    if (json is! Map) {
      throw EvaluationError('invalid condition node');
    }
    final map = Map<String, dynamic>.from(json);
    if (map.containsKey('all')) {
      return AllConditionNode(
        (map['all'] as List)
            .map((e) => ConditionNode.fromJson(e))
            .toList(),
      );
    }
    if (map.containsKey('any')) {
      return AnyConditionNode(
        (map['any'] as List)
            .map((e) => ConditionNode.fromJson(e))
            .toList(),
      );
    }
    if (map.containsKey('not')) {
      return NotConditionNode(ConditionNode.fromJson(map['not']));
    }
    return LeafConditionNode(Condition.fromJson(map));
  }
}

final class AllConditionNode extends ConditionNode {
  AllConditionNode(this.all);
  final List<ConditionNode> all;

  @override
  void collectLeaves(List<Condition> out) {
    for (final c in all) {
      c.collectLeaves(out);
    }
  }
}

final class AnyConditionNode extends ConditionNode {
  AnyConditionNode(this.any);
  final List<ConditionNode> any;

  @override
  void collectLeaves(List<Condition> out) {
    for (final c in any) {
      c.collectLeaves(out);
    }
  }
}

final class NotConditionNode extends ConditionNode {
  NotConditionNode(this.not);
  final ConditionNode not;

  @override
  void collectLeaves(List<Condition> out) => not.collectLeaves(out);
}

final class LeafConditionNode extends ConditionNode {
  LeafConditionNode(this.condition);
  final Condition condition;

  @override
  void collectLeaves(List<Condition> out) => out.add(condition);
}

class RuleEvent {
  RuleEvent({required this.id, required this.eventType, this.params = const {}});
  final String id;
  final String eventType;
  final Map<String, dynamic> params;

  String get pluginId => eventType;

  Map<String, dynamic> toJson() => {
        'id': id,
        'type': eventType,
        'params': params,
      };

  factory RuleEvent.fromJson(Map<String, dynamic> json) => RuleEvent(
        id: json['id'] as String,
        eventType: (json['type'] ?? json['plugin']) as String,
        params: Map<String, dynamic>.from(json['params'] as Map? ?? {}),
      );
}

typedef Action = RuleEvent;

ConditionNode conditionsFromV1When(List<Condition> when) => ConditionNode.all(
      when.map(ConditionNode.leaf).toList(),
    );
