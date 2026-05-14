use serde::{Deserialize, Serialize};
use uuid::Uuid;

use nauron_contracts::conditions::ConditionsEvaluateEvent;
use nauron_contracts::{IngestEvent, MirEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NauronCallbackStatus {
    InProgress,
    Success,
    Failure,
    Retryable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NauronCallbackEventType {
    MirProgress,
    MirResult,
    IngestProgress,
    IngestResult,
    ConditionsProgress,
    ConditionsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "engine", rename_all = "snake_case")]
pub enum NauronCallback {
    Mir {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: MirEvent,
    },
    Ingest {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: IngestEvent,
    },
    Conditions {
        event_type: NauronCallbackEventType,
        status: NauronCallbackStatus,
        job_id: Uuid,
        context_id: i64,
        event: ConditionsEvaluateEvent,
    },
}

#[cfg(test)]
mod tests {
    use super::{NauronCallback, NauronCallbackEventType, NauronCallbackStatus};
    use chrono::Utc;
    use nauron_contracts::{
        CompiledKnowledgeView, DossierArtifact, DossierMetadata, DossierRole, DossierScope,
        IngestEvent, IngestEvidenceAnchor, IngestFieldEvidence, IngestResult, KnowledgeArtifact,
        KnowledgeHint, SchemaVersion, TimelineView,
    };
    use uuid::Uuid;

    #[test]
    fn ingest_callback_carries_evidence_and_knowledge() {
        let job_id = Uuid::from_u128(1);
        let doc_id = Uuid::from_u128(2);
        let dossier_id = Uuid::from_u128(3);
        let callback = NauronCallback::Ingest {
            event_type: NauronCallbackEventType::IngestResult,
            status: NauronCallbackStatus::Success,
            job_id,
            context_id: 42,
            event: IngestEvent::Result(Box::new(IngestResult::Success {
                schema_version: SchemaVersion::default(),
                job_id,
                context_id: 42,
                data: Default::default(),
                evidence: vec![IngestFieldEvidence {
                    path: "rent_amount".to_string(),
                    anchors: vec![IngestEvidenceAnchor {
                        doc_id,
                        paragraph_id: "p1".to_string(),
                        quote: "The rent amount is 100 EUR.".to_string(),
                        explanation: None,
                    }],
                }],
                knowledge: Some(Box::new(KnowledgeArtifact {
                    dossier: DossierArtifact {
                        id: dossier_id,
                        context_id: 42,
                        name: "Agreement".to_string(),
                        role: DossierRole::RuntimeKnowledgeCompiler,
                        scope: DossierScope::Context,
                        revision: 1,
                        metadata: DossierMetadata {
                            require_conflicts_with: false,
                            max_conflict_nodes: None,
                        },
                    },
                    compiled_knowledge_view: CompiledKnowledgeView {
                        dossier_name: "Agreement".to_string(),
                        brief: "Agreement brief".to_string(),
                        active_surfaces: Vec::<KnowledgeHint>::new(),
                        temporal_hints: Vec::<KnowledgeHint>::new(),
                        conflict_hints: Vec::<KnowledgeHint>::new(),
                        retrieval_hints: Vec::<KnowledgeHint>::new(),
                    },
                    timeline_view: TimelineView {
                        nodes: Vec::new(),
                        edges: Vec::new(),
                    },
                })),
                language: "en".to_string(),
                tokens_used: None,
                completed_at: Utc::now(),
            })),
        };

        let NauronCallback::Ingest { event, .. } = callback else {
            panic!("expected ingest callback");
        };
        let IngestEvent::Result(result) = event else {
            panic!("expected ingest result event");
        };
        let IngestResult::Success {
            evidence,
            knowledge,
            ..
        } = result.as_ref()
        else {
            panic!("expected ingest success");
        };

        assert_eq!(evidence[0].path, "rent_amount");
        assert_eq!(knowledge.as_ref().unwrap().dossier.name, "Agreement");
        assert_eq!(knowledge.as_ref().unwrap().timeline_view.nodes.len(), 0);
    }
}
