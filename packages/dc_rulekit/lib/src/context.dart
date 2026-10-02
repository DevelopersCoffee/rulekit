class EvalContext {
  EvalContext({
    this.appNamespace = '',
    Map<String, dynamic>? facts,
    this.event,
  }) : facts = facts ?? {};

  final String appNamespace;
  final Map<String, dynamic> facts;
  final Map<String, dynamic>? event;

  EvalContext withFact(String key, dynamic value) {
    final next = Map<String, dynamic>.from(facts);
    next[key] = value;
    return EvalContext(
      appNamespace: appNamespace,
      facts: next,
      event: event,
    );
  }
}
