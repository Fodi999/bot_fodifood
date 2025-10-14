/// 🔍 AI Insight Events for WebSocket streaming
///
/// These events provide real-time visibility into AI processing pipeline:
/// - Intent classification
/// - Entity extraction
/// - Handler routing
/// - Processing metrics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🎯 AI Processing Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AIInsightEvent {
    /// Intent classification started
    IntentClassificationStarted {
        user_id: String,
        message: String,
        timestamp: String,
    },

    /// Intent classification completed
    IntentClassified {
        user_id: String,
        intent: String,
        confidence: f64,
        processing_time_ms: u64,
        timestamp: String,
    },

    /// Entity extraction in progress
    EntityExtraction {
        user_id: String,
        entities: Vec<ExtractedEntity>,
        timestamp: String,
    },

    /// Handler routing started
    HandlerRouting {
        user_id: String,
        intent: String,
        available_handlers: Vec<String>,
        timestamp: String,
    },

    /// Handler execution started
    HandlerExecutionStarted {
        user_id: String,
        handler_name: String,
        priority: u8,
        timestamp: String,
    },

    /// Handler execution completed
    HandlerExecutionCompleted {
        user_id: String,
        handler_name: String,
        success: bool,
        response_length: usize,
        processing_time_ms: u64,
        timestamp: String,
    },

    /// Context updated
    ContextUpdated {
        user_id: String,
        context_size: usize,
        metadata: HashMap<String, String>,
        timestamp: String,
    },

    /// Processing completed
    ProcessingCompleted {
        user_id: String,
        total_time_ms: u64,
        handlers_invoked: usize,
        timestamp: String,
    },

    /// Error occurred
    ProcessingError {
        user_id: String,
        error: String,
        stage: String,
        timestamp: String,
    },
}

/// 🧩 Extracted Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub entity_type: String,
    pub value: String,
    pub confidence: f64,
}

impl AIInsightEvent {
    /// Create intent classification started event
    pub fn classification_started(user_id: String, message: String) -> Self {
        Self::IntentClassificationStarted {
            user_id,
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create intent classified event
    pub fn classified(user_id: String, intent: String, confidence: f64, processing_time_ms: u64) -> Self {
        Self::IntentClassified {
            user_id,
            intent,
            confidence,
            processing_time_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create entity extraction event
    pub fn entity_extraction(user_id: String, entities: Vec<ExtractedEntity>) -> Self {
        Self::EntityExtraction {
            user_id,
            entities,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create handler routing event
    pub fn handler_routing(user_id: String, intent: String, available_handlers: Vec<String>) -> Self {
        Self::HandlerRouting {
            user_id,
            intent,
            available_handlers,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create handler execution started event
    pub fn handler_started(user_id: String, handler_name: String, priority: u8) -> Self {
        Self::HandlerExecutionStarted {
            user_id,
            handler_name,
            priority,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create handler execution completed event
    pub fn handler_completed(
        user_id: String,
        handler_name: String,
        success: bool,
        response_length: usize,
        processing_time_ms: u64,
    ) -> Self {
        Self::HandlerExecutionCompleted {
            user_id,
            handler_name,
            success,
            response_length,
            processing_time_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create context updated event
    pub fn context_updated(user_id: String, context_size: usize, metadata: HashMap<String, String>) -> Self {
        Self::ContextUpdated {
            user_id,
            context_size,
            metadata,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create processing completed event
    pub fn processing_completed(user_id: String, total_time_ms: u64, handlers_invoked: usize) -> Self {
        Self::ProcessingCompleted {
            user_id,
            total_time_ms,
            handlers_invoked,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create processing error event
    #[allow(dead_code)] // Will be used for error tracking
    pub fn processing_error(user_id: String, error: String, stage: String) -> Self {
        Self::ProcessingError {
            user_id,
            error,
            stage,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Convert event to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Convert event to pretty JSON string
    #[allow(dead_code)] // Used for debugging and logging
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = AIInsightEvent::classification_started(
            "user123".to_string(),
            "покажи меню".to_string(),
        );

        match event {
            AIInsightEvent::IntentClassificationStarted { user_id, message, .. } => {
                assert_eq!(user_id, "user123");
                assert_eq!(message, "покажи меню");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_event_serialization() {
        let event = AIInsightEvent::classified(
            "user123".to_string(),
            "show_menu".to_string(),
            0.95,
            45,
        );

        let json = event.to_json().unwrap();
        assert!(json.contains("intent_classified"));
        assert!(json.contains("show_menu"));
        assert!(json.contains("0.95"));
    }

    #[test]
    fn test_entity_extraction_event() {
        let entities = vec![
            ExtractedEntity {
                entity_type: "ingredient".to_string(),
                value: "лосось".to_string(),
                confidence: 0.9,
            },
        ];

        let event = AIInsightEvent::entity_extraction("user123".to_string(), entities);

        let json = event.to_json().unwrap();
        assert!(json.contains("entity_extraction"));
        assert!(json.contains("лосось"));
    }
}
