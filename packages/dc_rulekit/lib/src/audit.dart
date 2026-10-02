import 'model.dart';

abstract class AuditHook {
  void onReceipt(AuditReceipt receipt);
}

class NoopAuditHook implements AuditHook {
  @override
  void onReceipt(AuditReceipt receipt) {}
}
