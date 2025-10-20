//! 🤖 AI Agents Module - Multi-Agent System
//! 
//! This module provides the comprehensive multi-agent system with specialized
//! agents for different purposes: investment management, business operations,
//! and user interaction personalization.

pub mod memory_store;
pub mod investor_agent;
pub mod business_agent;
pub mod user_agent;

// Re-export commonly used types
pub use memory_store::{MemoryStore, MemoryQuery, MemorySortBy};
pub use investor_agent::InvestorAgent;
pub use business_agent::BusinessAgent;
pub use user_agent::UserAgent;

use anyhow::Result;
use crate::ai::agent_manager::{AIEntityAgent, AgentType};
use crate::ai::persistent_memory::PersistentMemory;
use std::sync::Arc;

/// Agent factory for creating different types of agents
pub struct AgentFactory;

impl AgentFactory {
    /// Create a new agent of the specified type
    pub async fn create_agent(
        agent_type: AgentType,
        agent_id: &str,
        persistent_memory: Arc<PersistentMemory>,
    ) -> Result<Box<dyn AIEntityAgent>> {
        match agent_type {
            AgentType::Investor => {
                let agent = InvestorAgent::new(agent_id, persistent_memory).await?;
                Ok(Box::new(agent))
            }
            AgentType::Business => {
                let agent = BusinessAgent::new(agent_id, persistent_memory).await?;
                Ok(Box::new(agent))
            }
            AgentType::User => {
                let agent = UserAgent::new(agent_id, persistent_memory).await?;
                Ok(Box::new(agent))
            }
            AgentType::General => {
                // For now, use UserAgent as the general agent
                let agent = UserAgent::new(agent_id, persistent_memory).await?;
                Ok(Box::new(agent))
            }
            AgentType::System => {
                // System agent for administrative tasks
                let agent = UserAgent::new(agent_id, persistent_memory).await?;
                Ok(Box::new(agent))
            }
        }
    }

    /// Get available agent types
    pub fn get_available_types() -> Vec<AgentType> {
        vec![
            AgentType::Investor,
            AgentType::Business,
            AgentType::User,
            AgentType::General,
        ]
    }

    /// Get agent type capabilities
    pub fn get_type_capabilities(agent_type: &AgentType) -> Vec<String> {
        match agent_type {
            AgentType::Investor => vec![
                "Portfolio Analysis".to_string(),
                "Investment Screening".to_string(),
                "Risk Assessment".to_string(),
                "Yield Calculation".to_string(),
                "Market Research".to_string(),
                "Performance Tracking".to_string(),
                "Investment Planning".to_string(),
                "Due Diligence".to_string(),
            ],
            AgentType::Business => vec![
                "Business Performance Analysis".to_string(),
                "Growth Campaign Management".to_string(),
                "Customer Analytics".to_string(),
                "Financial Planning".to_string(),
                "Market Research".to_string(),
                "KPI Tracking".to_string(),
                "Goal Setting & Monitoring".to_string(),
                "Competitive Intelligence".to_string(),
            ],
            AgentType::User => vec![
                "Personalized Communication".to_string(),
                "User Preference Learning".to_string(),
                "Conversation Context Management".to_string(),
                "Adaptive Response Styling".to_string(),
                "Interest Pattern Recognition".to_string(),
                "Communication Style Matching".to_string(),
                "User State Detection".to_string(),
                "Interaction Pattern Analysis".to_string(),
            ],
            AgentType::General => vec![
                "General Purpose Interaction".to_string(),
                "Adaptive Communication".to_string(),
                "Context Awareness".to_string(),
                "Multi-domain Knowledge".to_string(),
            ],
            AgentType::System => vec![
                "Administrative Tasks".to_string(),
                "System Management".to_string(),
                "Configuration Management".to_string(),
                "Resource Monitoring".to_string(),
            ],
        }
    }

    /// Get agent type description
    pub fn get_type_description(agent_type: &AgentType) -> String {
        match agent_type {
            AgentType::Investor => {
                "💼 Investment-focused AI agent specialized in portfolio management, \
                investment analysis, risk assessment, and financial planning. \
                Integrates with market data and provides data-driven investment insights."
            }
            AgentType::Business => {
                "🏢 Business operations AI agent focused on growth strategies, \
                customer analytics, financial performance, and operational efficiency. \
                Helps with business planning, KPI tracking, and competitive analysis."
            }
            AgentType::User => {
                "👤 User experience AI agent that provides personalized interactions \
                by learning user preferences, communication styles, and conversation patterns. \
                Adapts responses to match individual user needs and preferences."
            }
            AgentType::General => {
                "🤖 General purpose AI agent that can handle various types of interactions \
                and adapt to different contexts. Provides flexible, context-aware responses."
            }
            AgentType::System => {
                "⚙️ System administration AI agent specialized in administrative tasks, \
                system management, configuration, and resource monitoring. \
                Handles system-level operations and maintenance tasks."
            }
        }.to_string()
    }

    /// Validate agent configuration
    pub fn validate_agent_config(
        agent_type: &AgentType,
        agent_id: &str,
    ) -> Result<()> {
        // Validate agent ID format
        if agent_id.is_empty() {
            return Err(anyhow::anyhow!("Agent ID cannot be empty"));
        }

        if agent_id.len() > 50 {
            return Err(anyhow::anyhow!("Agent ID cannot be longer than 50 characters"));
        }

        // Validate agent ID contains only valid characters
        if !agent_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(anyhow::anyhow!(
                "Agent ID can only contain alphanumeric characters, hyphens, and underscores"
            ));
        }

        // Type-specific validations
        match agent_type {
            AgentType::Investor => {
                // Investor agents should have investment-related prefix
                if !agent_id.to_uppercase().contains("INV") && 
                   !agent_id.to_uppercase().contains("INVEST") &&
                   !agent_id.to_uppercase().contains("PORTFOLIO") {
                    log::warn!("Investor agent ID '{}' doesn't follow naming convention", agent_id);
                }
            }
            AgentType::Business => {
                // Business agents should have business-related prefix
                if !agent_id.to_uppercase().contains("BIZ") && 
                   !agent_id.to_uppercase().contains("BUSINESS") &&
                   !agent_id.to_uppercase().contains("GROWTH") {
                    log::warn!("Business agent ID '{}' doesn't follow naming convention", agent_id);
                }
            }
            AgentType::User => {
                // User agents should have user-related prefix
                if !agent_id.to_uppercase().contains("USER") && 
                   !agent_id.to_uppercase().contains("PERSONAL") &&
                   !agent_id.to_uppercase().contains("CLIENT") {
                    log::warn!("User agent ID '{}' doesn't follow naming convention", agent_id);
                }
            }
            AgentType::General => {
                // General agents can have any valid ID
            }
            AgentType::System => {
                // System agents should have system-related prefix
                if !agent_id.to_uppercase().contains("SYS") && 
                   !agent_id.to_uppercase().contains("SYSTEM") &&
                   !agent_id.to_uppercase().contains("ADMIN") {
                    log::warn!("System agent ID '{}' doesn't follow naming convention", agent_id);
                }
            }
        }

        Ok(())
    }

    /// Get recommended agent ID format
    pub fn get_recommended_id_format(agent_type: &AgentType) -> String {
        match agent_type {
            AgentType::Investor => "INV-{purpose}-{identifier}".to_string(),
            AgentType::Business => "BIZ-{purpose}-{identifier}".to_string(),
            AgentType::User => "USER-{name}-{identifier}".to_string(),
            AgentType::General => "GEN-{purpose}-{identifier}".to_string(),
            AgentType::System => "SYS-{purpose}-{identifier}".to_string(),
        }
    }

    /// Generate sample agent IDs for testing
    pub fn generate_sample_id(agent_type: &AgentType) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        match agent_type {
            AgentType::Investor => format!("INV-DEMO-{}", timestamp),
            AgentType::Business => format!("BIZ-DEMO-{}", timestamp),
            AgentType::User => format!("USER-DEMO-{}", timestamp),
            AgentType::General => format!("GEN-DEMO-{}", timestamp),
            AgentType::System => format!("SYS-DEMO-{}", timestamp),
        }
    }
}

/// Agent registry for managing agent types and metadata
pub struct AgentRegistry {
    /// Registered agent types and their metadata
    agent_types: Vec<AgentTypeInfo>,
}

/// Information about an agent type
#[derive(Debug, Clone)]
pub struct AgentTypeInfo {
    pub agent_type: AgentType,
    pub name: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub recommended_use_cases: Vec<String>,
    pub configuration_options: Vec<String>,
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRegistry {
    /// Create a new agent registry
    pub fn new() -> Self {
        let agent_types = vec![
            AgentTypeInfo {
                agent_type: AgentType::Investor,
                name: "Investment Agent".to_string(),
                description: AgentFactory::get_type_description(&AgentType::Investor),
                capabilities: AgentFactory::get_type_capabilities(&AgentType::Investor),
                recommended_use_cases: vec![
                    "Portfolio management and analysis".to_string(),
                    "Investment opportunity screening".to_string(),
                    "Risk assessment and management".to_string(),
                    "Financial planning and forecasting".to_string(),
                    "Market research and analysis".to_string(),
                ],
                configuration_options: vec![
                    "risk_tolerance: conservative|moderate|aggressive".to_string(),
                    "investment_focus: growth|income|balanced".to_string(),
                    "portfolio_size: small|medium|large".to_string(),
                    "analysis_depth: basic|detailed|comprehensive".to_string(),
                ],
            },
            AgentTypeInfo {
                agent_type: AgentType::Business,
                name: "Business Agent".to_string(),
                description: AgentFactory::get_type_description(&AgentType::Business),
                capabilities: AgentFactory::get_type_capabilities(&AgentType::Business),
                recommended_use_cases: vec![
                    "Business performance monitoring".to_string(),
                    "Growth strategy development".to_string(),
                    "Customer analytics and insights".to_string(),
                    "Financial planning and budgeting".to_string(),
                    "Competitive analysis".to_string(),
                ],
                configuration_options: vec![
                    "business_type: restaurant|retail|service|tech".to_string(),
                    "focus_area: growth|efficiency|customer_satisfaction".to_string(),
                    "reporting_frequency: daily|weekly|monthly".to_string(),
                    "analysis_scope: local|regional|national".to_string(),
                ],
            },
            AgentTypeInfo {
                agent_type: AgentType::User,
                name: "User Experience Agent".to_string(),
                description: AgentFactory::get_type_description(&AgentType::User),
                capabilities: AgentFactory::get_type_capabilities(&AgentType::User),
                recommended_use_cases: vec![
                    "Personalized user interactions".to_string(),
                    "Customer support and assistance".to_string(),
                    "User preference learning".to_string(),
                    "Adaptive communication styling".to_string(),
                    "Conversation context management".to_string(),
                ],
                configuration_options: vec![
                    "personalization_level: basic|advanced|expert".to_string(),
                    "communication_style: formal|casual|adaptive".to_string(),
                    "response_length: brief|normal|detailed".to_string(),
                    "learning_rate: slow|normal|fast".to_string(),
                ],
            },
            AgentTypeInfo {
                agent_type: AgentType::General,
                name: "General Purpose Agent".to_string(),
                description: AgentFactory::get_type_description(&AgentType::General),
                capabilities: AgentFactory::get_type_capabilities(&AgentType::General),
                recommended_use_cases: vec![
                    "General information assistance".to_string(),
                    "Multi-domain question answering".to_string(),
                    "Flexible interaction handling".to_string(),
                    "Context-aware responses".to_string(),
                ],
                configuration_options: vec![
                    "knowledge_depth: basic|intermediate|advanced".to_string(),
                    "response_style: informative|conversational|analytical".to_string(),
                    "specialization: none|domain_specific".to_string(),
                ],
            },
        ];

        Self { agent_types }
    }

    /// Get all registered agent types
    pub fn get_all_types(&self) -> &[AgentTypeInfo] {
        &self.agent_types
    }

    /// Get information about a specific agent type
    pub fn get_type_info(&self, agent_type: &AgentType) -> Option<&AgentTypeInfo> {
        self.agent_types.iter().find(|info| &info.agent_type == agent_type)
    }

    /// Get agent types suitable for a specific use case
    pub fn get_types_for_use_case(&self, use_case: &str) -> Vec<&AgentTypeInfo> {
        self.agent_types
            .iter()
            .filter(|info| {
                info.recommended_use_cases
                    .iter()
                    .any(|uc| uc.to_lowercase().contains(&use_case.to_lowercase()))
            })
            .collect()
    }

    /// Get agent types with a specific capability
    pub fn get_types_with_capability(&self, capability: &str) -> Vec<&AgentTypeInfo> {
        self.agent_types
            .iter()
            .filter(|info| {
                info.capabilities
                    .iter()
                    .any(|cap| cap.to_lowercase().contains(&capability.to_lowercase()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_factory_types() {
        let types = AgentFactory::get_available_types();
        assert!(types.contains(&AgentType::Investor));
        assert!(types.contains(&AgentType::Business));
        assert!(types.contains(&AgentType::User));
        assert!(types.contains(&AgentType::General));
    }

    #[test]
    fn test_agent_id_validation() {
        // Valid IDs
        assert!(AgentFactory::validate_agent_config(&AgentType::Investor, "INV-TEST-1").is_ok());
        assert!(AgentFactory::validate_agent_config(&AgentType::Business, "BIZ-DEMO").is_ok());
        assert!(AgentFactory::validate_agent_config(&AgentType::User, "USER-123").is_ok());

        // Invalid IDs
        assert!(AgentFactory::validate_agent_config(&AgentType::Investor, "").is_err());
        assert!(AgentFactory::validate_agent_config(&AgentType::Business, "a".repeat(51).as_str()).is_err());
        assert!(AgentFactory::validate_agent_config(&AgentType::User, "user@domain.com").is_err());
    }

    #[test]
    fn test_agent_registry() {
        let registry = AgentRegistry::new();
        
        assert_eq!(registry.get_all_types().len(), 4);
        
        let investor_info = registry.get_type_info(&AgentType::Investor);
        assert!(investor_info.is_some());
        assert_eq!(investor_info.unwrap().name, "Investment Agent");
        
        let portfolio_types = registry.get_types_for_use_case("portfolio");
        assert!(!portfolio_types.is_empty());
        
        let analysis_types = registry.get_types_with_capability("analysis");
        assert!(!analysis_types.is_empty());
    }

    #[tokio::test]
    async fn test_agent_creation() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_agents.db").unwrap());
        
        // Test creating different agent types
        let investor_agent = AgentFactory::create_agent(
            AgentType::Investor,
            "INV-TEST",
            persistent_memory.clone()
        ).await;
        assert!(investor_agent.is_ok());

        let business_agent = AgentFactory::create_agent(
            AgentType::Business,
            "BIZ-TEST",
            persistent_memory.clone()
        ).await;
        assert!(business_agent.is_ok());

        let user_agent = AgentFactory::create_agent(
            AgentType::User,
            "USER-TEST",
            persistent_memory.clone()
        ).await;
        assert!(user_agent.is_ok());
    }
}