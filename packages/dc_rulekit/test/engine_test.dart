import 'package:dc_rulekit/dc_rulekit.dart';
import 'package:test/test.dart';

class AlwaysCondition implements ConditionEvaluator {
  @override
  String get pluginId => 'demo.when.always';
  @override
  bool evaluate(Map<String, dynamic> params, EvalContext ctx) => true;
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

void main() {
  PluginRegistry registry() {
    final reg = PluginRegistry();
    reg.registerCondition(AlwaysCondition());
    reg.registerAction(LogAction([]));
    reg.registerAction(PureEcho());
    return reg;
  }

  test('evaluate runs actions when conditions pass', () {
    final log = <String>[];
    final reg = PluginRegistry();
    reg.registerCondition(AlwaysCondition());
    reg.registerAction(LogAction(log));
    final engine = Engine(reg);
    final rule = Rule(
      id: 'demo.app/rule-1',
      title: 'Test',
      source: RuleSource.static,
      when: [Condition(id: 'c1', plugin: 'demo.when.always')],
      then: [
        Action(id: 'a1', plugin: 'demo.then.log', params: {'message': 'hello'}),
      ],
    );
    final receipt = engine.evaluate(rule, EvalContext(appNamespace: 'demo.app'));
    expect(receipt.matched, isTrue);
    expect(log, ['hello']);
  });

  test('dry-run skips impure actions', () {
    final log = <String>[];
    final reg = PluginRegistry();
    reg.registerCondition(AlwaysCondition());
    reg.registerAction(LogAction(log));
    final engine = Engine(reg);
    final rule = Rule(
      id: 'demo.app/rule-2',
      title: 'Dry',
      source: RuleSource.static,
      when: [Condition(id: 'c1', plugin: 'demo.when.always')],
      then: [Action(id: 'a1', plugin: 'demo.then.log')],
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
    final engine = Engine(registry());
    final rule = Rule(
      id: 'demo.app/bad',
      title: 'Bad',
      source: RuleSource.static,
      when: [Condition(id: 'c1', plugin: 'missing.plugin')],
    );
    expect(
      () => engine.evaluate(rule, EvalContext(appNamespace: 'demo.app')),
      throwsA(isA<UnknownConditionPlugin>()),
    );
  });

  test('proposal approve lifecycle', () {
    final proposals = ProposalStore();
    final active = RuleStore();
    final rule = Rule(
      id: 'demo.app/prop',
      title: 'Prop',
      source: RuleSource.static,
    );
    final proposal = proposals.propose(rule);
    expect(proposal.status, ProposalStatus.proposed);
    proposals.approve(proposal.proposalId, active);
    expect(active.get('demo.app/prop').title, 'Prop');
  });

  test('audit hook receives opaque payload', () {
    final reg = PluginRegistry();
    reg.registerCondition(AlwaysCondition());
    reg.registerAction(PureEcho());
    final engine = Engine(reg);
    final rule = Rule(
      id: 'demo.app/audit',
      title: 'Audit',
      source: RuleSource.static,
      when: [Condition(id: 'c1', plugin: 'demo.when.always')],
      then: [Action(id: 'a1', plugin: 'demo.then.echo')],
    );
    AuditReceipt? captured;
    final hook = _CaptureHook((r) => captured = r);
    engine.evaluateWithAudit(
      rule,
      EvalContext(appNamespace: 'demo.app'),
      const EvaluateOptions(),
      hook,
      {'host': 'payload'},
    );
    expect(captured!.opaque, {'host': 'payload'});
  });
}

class _CaptureHook implements AuditHook {
  _CaptureHook(this._fn);
  final void Function(AuditReceipt) _fn;
  @override
  void onReceipt(AuditReceipt receipt) => _fn(receipt);
}
