use crate::model::AuditReceipt;

/// Host hook invoked after each evaluation with an opaque receipt payload.
pub trait AuditHook: Send + Sync {
    fn on_receipt(&self, receipt: &AuditReceipt);
}

#[derive(Default)]
pub struct NoopAuditHook;

impl AuditHook for NoopAuditHook {
    fn on_receipt(&self, _receipt: &AuditReceipt) {}
}
