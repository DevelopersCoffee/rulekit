import 'dart:convert';
import 'dart:io';

import 'error.dart';
import 'model.dart';
import 'store.dart';

class ProposalStore {
  ProposalStore({this.filePath});

  final String? filePath;
  final _proposals = <String, RuleProposal>{};

  RuleProposal propose(Rule rule) {
    rule.validateSchema();
    final proposal = RuleProposal(
      proposalId: _uuid(),
      status: ProposalStatus.proposed,
      proposedAt: DateTime.now().toUtc(),
      rule: rule,
    );
    _proposals[proposal.proposalId] = proposal;
    _persist();
    return proposal;
  }

  RuleProposal get(String proposalId) {
    final p = _proposals[proposalId];
    if (p == null) throw ProposalNotFound(proposalId);
    return p;
  }

  RuleProposal reject(String proposalId) {
    final proposal = get(proposalId);
    if (proposal.status != ProposalStatus.proposed) {
      throw InvalidProposalState('cannot reject proposal in state ${proposal.status.name}');
    }
    final updated = RuleProposal(
      proposalId: proposal.proposalId,
      status: ProposalStatus.rejected,
      proposedAt: proposal.proposedAt,
      rule: proposal.rule,
    );
    _proposals[proposalId] = updated;
    _persist();
    return updated;
  }

  Rule approve(String proposalId, RuleStore activeStore) {
    final proposal = get(proposalId);
    if (proposal.status != ProposalStatus.proposed) {
      throw InvalidProposalState('cannot approve proposal in state ${proposal.status.name}');
    }
    final updated = RuleProposal(
      proposalId: proposal.proposalId,
      status: ProposalStatus.approved,
      proposedAt: proposal.proposedAt,
      rule: proposal.rule,
    );
    _proposals[proposalId] = updated;
    activeStore.upsert(proposal.rule);
    _persist();
    return proposal.rule;
  }

  void _persist() {
    final path = filePath;
    if (path == null) return;
    final file = File(path);
    file.parent.createSync(recursive: true);
    final data = _proposals.values.map((p) => p.toJson()).toList();
    file.writeAsStringSync(const JsonEncoder.withIndent('  ').convert(data));
  }

  String _uuid() {
    // Lightweight v4-like id without extra deps.
    final now = DateTime.now().microsecondsSinceEpoch;
    return 'prop-$now-${now.hashCode.abs()}';
  }
}
