import 'dart:convert';
import 'dart:io';

import 'package:dc_rulekit/dc_rulekit.dart';
import 'package:test/test.dart';

class AlwaysCondition implements ConditionEvaluator {
  @override
  String get pluginId => 'demo.when.always';
  @override
  bool evaluate(Map<String, dynamic> params, EvalContext ctx) => true;
}

class SchemaCondition implements ConditionEvaluator, ParamsSchemaProvider {
  @override
  String get pluginId => 'demo.when.schema';
  @override
  bool evaluate(Map<String, dynamic> params, EvalContext ctx) => true;
  @override
  Map<String, dynamic>? get paramsSchema => {
        'type': 'object',
        'required': ['min'],
        'properties': {'min': {'type': 'number'}},
        'additionalProperties': false,
      };
}

class LogAction implements ActionHandler {
  LogAction(this.log);
  final List<String> log;
  @override
  String get pluginId => 'demo.then.log';
  @override
  bool get isPure => false;
  @override
  Map<String, dynamic> execute(Map<String, dynamic> params, EvalContext ctx) {
    log.add(params['message'] as String? ?? 'log');
    return {'logged': params['message']};
  }
}

class PureEcho implements ActionHandler {
  @override
  String get pluginId => 'demo.then.echo';
  @override
  bool get isPure => true;
  @override
  Map<String, dynamic> execute(Map<String, dynamic> params, EvalContext ctx) => params;
}

class _CaptureHook implements AuditHook {
  _CaptureHook(this._fn);
  final void Function(AuditReceipt) _fn;
  @override
  void onReceipt(AuditReceipt receipt) => _fn(receipt);
}

PluginRegistry demoRegistry({List<String>? log}) {
  final reg = PluginRegistry();
  reg.registerCondition(AlwaysCondition());
  reg.registerCondition(SchemaCondition());
  reg.registerAction(LogAction(log ?? []));
  reg.registerAction(PureEcho());
  return reg;
}

void main() {
  test('evaluate runs actions when conditions pass', () {
    final log = <String>[];
    final reg = demoRegistry(log: log);
    final engine = Engine(reg);
    final rule = Rule(
      id: 'demo.app/rule-1',
      title: 'Test',
      source: RuleSource.static,
      conditions: ConditionNode.all([
        ConditionNode.leaf(Condition(id: 'c1', plugin: 'demo.when.always')),
      ]),
      events: [
        RuleEvent(
          id: 'a1',
          eventType: 'demo.then.log',
          params: {'message': 'hello'},
        ),
      ],
    );
    final receipt = engine.evaluate(rule, EvalContext(appNamespace: 'demo.app'));
    expect(receipt.matched, isTrue);
    expect(log, ['hello']);
  });

  test('dry-run skips impure actions', () {
    final log = <String>[];
    final engine = Engine(demoRegistry(log: log));
    final rule = Rule(
      id: 'demo.app/rule-2',
      title: 'Dry',
      source: RuleSource.static,
      conditions: ConditionNode.all([
        ConditionNode.leaf(Condition(id: 'c1', plugin: 'demo.when.always')),
      ]),
      events: [RuleEvent(id: 'a1', eventType: 'demo.then.log')],
    );
    final receipt = engine.evaluate(
      rule,
      EvalContext(appNamespace: 'demo.app'),
      const EvaluateOptions(dryRun: true),
    );
    expect(receipt.actionOutcomes.first.skippedDryRun, isTrue);
    expect(log, isEmpty);
  });

  test('unknown plugin fail-closed', () {
    final engine = Engine(demoRegistry());
    final rule = Rule(
      id: 'demo.app/bad',
      title: 'Bad',
      source: RuleSource.static,
      conditions: ConditionNode.all([
        ConditionNode.leaf(Condition(id: 'c1', plugin: 'missing.plugin')),
      ]),
    );
    expect(
      () => engine.evaluate(rule, EvalContext(appNamespace: 'demo.app')),
      throwsA(isA<UnknownConditionPlugin>()),
    );
  });

  test('proposal approve lifecycle', () {
    final reg = demoRegistry();
    final proposals = ProposalStore();
    final active = RuleStore();
    final rule = Rule(
      id: 'demo.app/prop',
      title: 'Prop',
      source: RuleSource.static,
    );
    final proposal = proposals.propose(rule, reg);
    expect(proposal.status, ProposalStatus.proposed);
    proposals.approve(proposal.proposalId, active);
    expect(active.get('demo.app/prop').title, 'Prop');
  });

  test('audit hook receives opaque payload', () {
    final reg = demoRegistry();
    final engine = Engine(reg);
    final rule = Rule(
      id: 'demo.app/audit',
      title: 'Audit',
      source: RuleSource.static,
      conditions: ConditionNode.all([
        ConditionNode.leaf(Condition(id: 'c1', plugin: 'demo.when.always')),
      ]),
      events: [RuleEvent(id: 'a1', eventType: 'demo.then.echo')],
    );
    AuditReceipt? captured;
    engine.evaluateWithAudit(
      rule,
      EvalContext(appNamespace: 'demo.app'),
      const EvaluateOptions(),
      _CaptureHook((r) => captured = r),
      {'host': 'payload'},
    );
    expect(captured!.opaque, {'host': 'payload'});
  });

  test('v1 json compat', () {
    final log = <String>[];
    final engine = Engine(demoRegistry(log: log));
    final v1 = jsonDecode('''
{
  "schema_version": 1,
  "id": "demo.app/v1",
  "title": "Legacy",
  "source": "static",
  "when": [{"id": "c1", "plugin": "demo.when.always", "params": {}}],
  "then": [{"id": "a1", "plugin": "demo.then.log", "params": {"message": "legacy"}}]
}
''') as Map<String, dynamic>;
    final rule = Rule.fromJson(v1);
    expect(rule.schemaVersion, 2);
    engine.evaluate(rule, EvalContext(appNamespace: 'demo.app'));
    expect(log, ['legacy']);
  });

  test('golden v2 fixture roundtrip', () {
    final path = '../../schema/fixtures/v2_basic_rule.json';
    final text = File(path).readAsStringSync();
    final rule = Rule.fromJson(jsonDecode(text) as Map<String, dynamic>);
    expect(rule.events.first.eventType, 'demo.then.log');
    expect(rule.conditions.leaves().first.plugin, 'demo.when.threshold');
  });

  test('propose validates params schema', () {
    final reg = demoRegistry();
    final proposals = ProposalStore();
    final rule = Rule(
      id: 'demo.app/bad-schema',
      title: 'Bad',
      source: RuleSource.static,
      conditions: ConditionNode.all([
        ConditionNode.leaf(Condition(
          id: 'c1',
          plugin: 'demo.when.schema',
          params: {'wrong': true},
        )),
      ]),
    );
    expect(
      () => proposals.propose(rule, reg),
      throwsA(isA<InvalidPluginParams>()),
    );
  });
}
