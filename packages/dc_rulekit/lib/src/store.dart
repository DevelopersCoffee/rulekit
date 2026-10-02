import 'dart:convert';
import 'dart:io';

import 'error.dart';
import 'model.dart';

class RuleStore {
  RuleStore({this.filePath});

  final String? filePath;
  final _rules = <String, Rule>{};

  static RuleStore loadFromDisk(String path) {
    final store = RuleStore(filePath: path);
    final file = File(path);
    if (file.existsSync()) {
      final list = jsonDecode(file.readAsStringSync()) as List;
      for (final item in list) {
        final rule = Rule.fromJson(Map<String, dynamic>.from(item as Map));
        rule.validateSchema();
        store._rules[rule.id] = rule;
      }
    }
    return store;
  }

  void persist() {
    final path = filePath;
    if (path == null) return;
    final file = File(path);
    file.parent.createSync(recursive: true);
    final data = _rules.values.map((r) => r.toJson()).toList();
    file.writeAsStringSync(const JsonEncoder.withIndent('  ').convert(data));
  }

  void upsert(Rule rule) {
    rule.validateSchema();
    _rules[rule.id] = rule;
    persist();
  }

  Rule get(String ruleId) {
    final rule = _rules[ruleId];
    if (rule == null) throw RuleNotFound(ruleId);
    return rule;
  }

  Rule remove(String ruleId) {
    final rule = _rules.remove(ruleId);
    if (rule == null) throw RuleNotFound(ruleId);
    persist();
    return rule;
  }

  List<Rule> list() {
    final rules = _rules.values.toList()..sort((a, b) => a.id.compareTo(b.id));
    return rules;
  }

  int get length => _rules.length;
}
