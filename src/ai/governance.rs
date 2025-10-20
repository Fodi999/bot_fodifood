//! 🎭 AI Governance Layer
//! 
//! Meta-agent system for monitoring and governing AI agent performance.
//! Automatically adjusts strategies based on performance patterns and
//! ensures optimal system-wide decision making.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, interval};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::ai::SharedBus;
use crate::ai::agent_state::AgentStateManager;
use crate::ai::business_economy_loop::{BusinessEconomyLoop, CyclePerformance};
use crate::ai::shared_bus::MessageType;

/// AI Governance Layer for meta-management of agent ecosystem
pub struct AIGovernanceLayer {
    /// Shared communication bus
    bus: Arc<SharedBus>,
    /// Agent state manager
    state_manager: Arc<AgentStateManager>,
    /// Business economy loop reference
    economy_loop: Option<Arc<BusinessEconomyLoop>>,
    /// Governance configuration
    config: GovernanceConfig,
    /// Performance tracking
    performance_tracker: Arc<tokio::sync::RwLock<PerformanceTracker>>,
    /// Strategy adjustments history
    adjustment_history: Arc<tokio::sync::RwLock<Vec<StrategyAdjustment>>>,
    /// 🧠 SELF-LEARNING: Current strategy weights
    strategy_weights: Arc<tokio::sync::RwLock<StrategyWeights>>,
    /// 🧠 SELF-LEARNING: Learning data from past decisions
    learning_data: Arc<tokio::sync::RwLock<LearningData>>,
}

/// Configuration for governance behavior
#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    /// Minimum ROI threshold before intervention
    pub min_roi_threshold: f64,
    /// Number of poor cycles before major strategy change
    pub poor_cycle_threshold: u32,
    /// Maximum allowed variance in agent performance
    pub max_performance_variance: f64,
    /// Monitoring interval in hours
    pub monitoring_interval_hours: u64,
    /// Auto-adjustment enabled
    pub auto_adjustment_enabled: bool,
    /// Risk tolerance for strategy changes
    pub risk_tolerance: RiskTolerance,
}

/// Risk tolerance levels for governance decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskTolerance {
    Conservative, // Minimal changes, high stability
    Moderate,     // Balanced approach
    Aggressive,   // Rapid adjustments, high optimization
}

/// Performance tracking across all agents and cycles
#[derive(Debug, Clone)]
pub struct PerformanceTracker {
    /// ROI trend over recent cycles
    pub roi_trend: Vec<f64>,
    /// Agent performance trends
    pub agent_trends: HashMap<String, Vec<f64>>,
    /// System-wide KPIs
    pub system_kpis: SystemKPIs,
    /// Last governance action timestamp
    pub last_action_at: DateTime<Utc>,
    /// Consecutive poor cycles count
    pub consecutive_poor_cycles: u32,
}

/// System-wide Key Performance Indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemKPIs {
    /// Overall system efficiency score
    pub efficiency_score: f64,
    /// Agent coordination quality
    pub coordination_quality: f64,
    /// Decision consistency across agents
    pub decision_consistency: f64,
    /// System stability metric
    pub stability_score: f64,
    /// Resource utilization efficiency
    pub resource_efficiency: f64,
}

/// Record of strategy adjustments made by governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAdjustment {
    /// Unique adjustment identifier
    pub adjustment_id: String,
    /// Type of adjustment made
    pub adjustment_type: AdjustmentType,
    /// Trigger that caused this adjustment
    pub trigger: GovernanceTrigger,
    /// Affected agents
    pub affected_agents: Vec<String>,
    /// Old vs new strategy parameters
    pub strategy_changes: HashMap<String, StrategyChange>,
    /// Expected impact of the adjustment
    pub expected_impact: ExpectedImpact,
    /// Actual measured impact (filled later)
    pub actual_impact: Option<MeasuredImpact>,
    /// Adjustment timestamp
    pub adjusted_at: DateTime<Utc>,
}

/// Types of strategic adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    /// Rebalance investment allocation
    InvestmentRebalancing,
    /// Change business strategy focus
    StrategyPivot,
    /// Adjust marketing approach
    MarketingOptimization,
    /// Modify risk parameters
    RiskAdjustment,
    /// Replace underperforming agent
    AgentReplacement,
    /// Fine-tune coordination parameters
    CoordinationTuning,
}

/// Triggers that cause governance intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTrigger {
    /// ROI below threshold for consecutive cycles
    ConsistentPoorROI { cycles: u32, threshold: f64 },
    /// Single agent consistently underperforming
    AgentUnderperformance { agent_id: String, score: f64 },
    /// High variance in system performance
    PerformanceInstability { variance: f64 },
    /// External market conditions change
    MarketShift { condition: String },
    /// Scheduled optimization review
    ScheduledReview,
}

/// Change made to a strategy parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyChange {
    /// Parameter that was changed
    pub parameter: String,
    /// Old value
    pub old_value: serde_json::Value,
    /// New value
    pub new_value: serde_json::Value,
    /// Reason for change
    pub reason: String,
}

/// Expected impact of an adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedImpact {
    /// Expected ROI improvement
    pub roi_improvement: f64,
    /// Expected efficiency gain
    pub efficiency_gain: f64,
    /// Expected risk reduction
    pub risk_reduction: f64,
    /// Timeline for impact measurement
    pub measurement_timeline_days: u32,
}

/// Measured impact after adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasuredImpact {
    /// Actual ROI change
    pub roi_change: f64,
    /// Actual efficiency change
    pub efficiency_change: f64,
    /// Actual risk change
    pub risk_change: f64,
    /// Impact measurement timestamp
    pub measured_at: DateTime<Utc>,
    /// Whether adjustment was successful
    pub success: bool,
}

/// 🧠 SELF-LEARNING SYSTEM: Strategy weights management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyWeights {
    /// Marketing budget allocation weight (0.0 - 1.0)
    pub marketing_weight: f64,
    /// Investment allocation weight (0.0 - 1.0)  
    pub investment_weight: f64,
    /// Business development weight (0.0 - 1.0)
    pub business_dev_weight: f64,
    /// Risk management weight (0.0 - 1.0)
    pub risk_management_weight: f64,
    /// User acquisition weight (0.0 - 1.0)
    pub user_acquisition_weight: f64,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Learning confidence (how sure we are about these weights)
    pub confidence_score: f64,
}

impl Default for StrategyWeights {
    fn default() -> Self {
        Self {
            marketing_weight: 0.25,
            investment_weight: 0.30,
            business_dev_weight: 0.20,
            risk_management_weight: 0.15,
            user_acquisition_weight: 0.10,
            updated_at: Utc::now(),
            confidence_score: 0.5, // Start with medium confidence
        }
    }
}

/// Learning from past adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningData {
    /// Historical strategy effectiveness
    pub strategy_effectiveness: HashMap<String, f64>,
    /// Optimal allocation patterns discovered
    pub optimal_patterns: Vec<AllocationPattern>,
    /// Performance predictors 
    pub performance_predictors: HashMap<String, f64>,
    /// Market condition responses
    pub market_responses: HashMap<String, StrategyWeights>,
    /// Last learning update
    pub last_learning_update: DateTime<Utc>,
}

/// Discovered allocation pattern that works well
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationPattern {
    /// Pattern name/description
    pub name: String,
    /// Strategy weights for this pattern
    pub weights: StrategyWeights,
    /// Market conditions when this pattern works best
    pub optimal_conditions: Vec<String>,
    /// Success rate of this pattern
    pub success_rate: f64,
    /// Average ROI when using this pattern
    pub average_roi: f64,
}

/// Resource reallocation instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReallocation {
    /// Source strategy to reduce
    pub from_strategy: String,
    /// Target strategy to boost
    pub to_strategy: String,
    /// Amount to transfer (percentage)
    pub transfer_amount: f64,
    /// Reason for reallocation
    pub reason: String,
    /// Expected improvement
    pub expected_improvement: f64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            min_roi_threshold: 0.05, // 5% minimum ROI
            poor_cycle_threshold: 3, // 3 consecutive poor cycles
            max_performance_variance: 0.3, // 30% max variance
            monitoring_interval_hours: 6, // Monitor every 6 hours
            auto_adjustment_enabled: true,
            risk_tolerance: RiskTolerance::Moderate,
        }
    }
}

impl Default for SystemKPIs {
    fn default() -> Self {
        Self {
            efficiency_score: 0.75,
            coordination_quality: 0.80,
            decision_consistency: 0.70,
            stability_score: 0.85,
            resource_efficiency: 0.72,
        }
    }
}

impl AIGovernanceLayer {
    /// Create new governance layer
    pub async fn new(
        bus: Arc<SharedBus>,
        state_manager: Arc<AgentStateManager>,
        config: Option<GovernanceConfig>,
    ) -> Result<Self> {
        let performance_tracker = Arc::new(tokio::sync::RwLock::new(PerformanceTracker {
            roi_trend: Vec::new(),
            agent_trends: HashMap::new(),
            system_kpis: SystemKPIs::default(),
            last_action_at: Utc::now(),
            consecutive_poor_cycles: 0,
        }));

        Ok(Self {
            bus,
            state_manager,
            economy_loop: None,
            config: config.unwrap_or_default(),
            performance_tracker,
            adjustment_history: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            strategy_weights: Arc::new(tokio::sync::RwLock::new(StrategyWeights::default())),
            learning_data: Arc::new(tokio::sync::RwLock::new(LearningData {
                strategy_effectiveness: HashMap::new(),
                optimal_patterns: Vec::new(),
                performance_predictors: HashMap::new(),
                market_responses: HashMap::new(),
                last_learning_update: Utc::now(),
            })),
        })
    }

    /// Set reference to business economy loop for monitoring
    pub fn set_economy_loop(&mut self, economy_loop: Arc<BusinessEconomyLoop>) {
        self.economy_loop = Some(economy_loop);
    }

    /// Start continuous governance monitoring
    pub async fn start_governance_monitoring(&self) -> Result<()> {
        if !self.config.auto_adjustment_enabled {
            tracing::info!("🎭 Governance monitoring started in observation-only mode");
        } else {
            tracing::info!("🎭 Governance monitoring started with auto-adjustment enabled");
        }

        let mut monitoring_interval = interval(Duration::from_secs(
            self.config.monitoring_interval_hours * 3600
        ));

        loop {
            monitoring_interval.tick().await;
            
            match self.perform_governance_check().await {
                Ok(_) => {
                    tracing::debug!("✅ Governance check completed successfully");
                },
                Err(e) => {
                    tracing::error!("❌ Governance check failed: {}", e);
                }
            }
        }
    }

    /// Perform comprehensive governance check
    pub async fn perform_governance_check(&self) -> Result<()> {
        tracing::info!("🔍 Performing governance analysis...");

        // 1. Collect current performance data
        let performance_data = self.collect_performance_data().await?;
        
        // 2. 🧠 SELF-LEARNING: Auto-adjust strategy weights based on performance
        let current_efficiency = performance_data.calculate_efficiency();
        let current_roi = performance_data.calculate_roi();
        
        if self.config.auto_adjustment_enabled {
            let reallocations = self.auto_adjust_strategy_weights(current_efficiency, current_roi).await?;
            
            // Apply reallocations to agents if any were generated
            if !reallocations.is_empty() {
                tracing::info!("🔄 Applying {} resource reallocations", reallocations.len());
                self.apply_resource_reallocations(reallocations).await?;
            }
        }
        
        // 3. Analyze trends and identify issues
        let issues = self.analyze_performance_trends(&performance_data).await?;
        
        // 4. Determine if intervention is needed
        if !issues.is_empty() {
            tracing::warn!("⚠️ Governance identified {} performance issues", issues.len());
            
            if self.config.auto_adjustment_enabled {
                // 5. Execute strategic adjustments
                for issue in issues {
                    self.execute_strategic_adjustment(issue).await?;
                }
            } else {
                // 6. Send recommendations without auto-adjustment
                self.send_governance_recommendations(issues).await?;
            }
        } else {
            tracing::info!("✅ System performance within acceptable parameters");
        }

        // 7. Update system KPIs
        self.update_system_kpis().await?;

        Ok(())
    }

    /// Collect current performance data from all agents
    async fn collect_performance_data(&self) -> Result<PerformanceData> {
        let mut performance_data = PerformanceData::new();

        // Get agent performance from state manager
        let comparison = self.state_manager.get_performance_comparison().await?;
        performance_data.agent_performance = comparison;

        // Get economy loop performance if available
        if let Some(economy_loop) = &self.economy_loop {
            let history = economy_loop.get_performance_history().await;
            performance_data.cycle_history = history;
            
            let health_trend = economy_loop.get_health_trend().await;
            performance_data.health_trend = health_trend;
        }

        // Get bus statistics
        let bus_stats = self.bus.get_stats().await;
        performance_data.communication_health = bus_stats.total_messages as f64 / 
            (bus_stats.uptime_seconds as f64 / 3600.0).max(1.0); // Messages per hour

        Ok(performance_data)
    }

    /// Analyze performance trends and identify issues
    async fn analyze_performance_trends(&self, data: &PerformanceData) -> Result<Vec<GovernanceTrigger>> {
        let mut issues = Vec::new();

        // Check for consistent poor ROI
        if data.cycle_history.len() >= self.config.poor_cycle_threshold as usize {
            let recent_cycles = &data.cycle_history[data.cycle_history.len() - self.config.poor_cycle_threshold as usize..];
            let poor_roi_count = recent_cycles.iter()
                .filter(|cycle| cycle.roi < self.config.min_roi_threshold)
                .count();

            if poor_roi_count == self.config.poor_cycle_threshold as usize {
                issues.push(GovernanceTrigger::ConsistentPoorROI {
                    cycles: self.config.poor_cycle_threshold,
                    threshold: self.config.min_roi_threshold,
                });

                // Update tracker
                let mut tracker = self.performance_tracker.write().await;
                tracker.consecutive_poor_cycles = self.config.poor_cycle_threshold;
                drop(tracker);
            }
        }

        // Check for agent underperformance
        for (agent_id, performance) in &data.agent_performance {
            if performance.success_rate < 0.6 || performance.accuracy_score < 0.5 {
                issues.push(GovernanceTrigger::AgentUnderperformance {
                    agent_id: agent_id.clone(),
                    score: performance.success_rate,
                });
            }
        }

        // Check for performance instability
        if data.health_trend.len() >= 5 {
            let recent_values = &data.health_trend[data.health_trend.len() - 5..];
            let mean = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
            let variance = recent_values.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / recent_values.len() as f64;

            if variance > self.config.max_performance_variance {
                issues.push(GovernanceTrigger::PerformanceInstability { variance });
            }
        }

        Ok(issues)
    }

    /// Execute strategic adjustment based on identified issue
    async fn execute_strategic_adjustment(&self, trigger: GovernanceTrigger) -> Result<()> {
        let adjustment_id = uuid::Uuid::new_v4().to_string();
        
        tracing::info!("🔧 Executing strategic adjustment for: {:?}", trigger);

        let adjustment = match &trigger {
            GovernanceTrigger::ConsistentPoorROI { cycles, threshold } => {
                self.adjust_investment_strategy(*cycles, *threshold).await?
            },
            GovernanceTrigger::AgentUnderperformance { agent_id, score } => {
                self.optimize_agent_performance(agent_id, *score).await?
            },
            GovernanceTrigger::PerformanceInstability { variance } => {
                self.stabilize_system_performance(*variance).await?
            },
            _ => {
                // Default generic optimization
                self.perform_generic_optimization().await?
            }
        };

        // Record the adjustment
        let mut history = self.adjustment_history.write().await;
        history.push(StrategyAdjustment {
            adjustment_id: adjustment_id.clone(),
            adjustment_type: adjustment.adjustment_type.clone(),
            trigger,
            affected_agents: adjustment.affected_agents.clone(),
            strategy_changes: adjustment.strategy_changes,
            expected_impact: adjustment.expected_impact,
            actual_impact: None,
            adjusted_at: Utc::now(),
        });

        // Keep only last 50 adjustments
        if history.len() > 50 {
            history.remove(0);
        }
        drop(history);

        // Broadcast adjustment notification
        self.bus.broadcast(
            "GOVERNANCE",
            "strategy_adjustment",
            MessageType::Alert,
            json!({
                "adjustment_id": adjustment_id,
                "type": adjustment.adjustment_type,
                "affected_agents": adjustment.affected_agents,
                "timestamp": Utc::now()
            })
        ).await?;

        Ok(())
    }

    /// Adjust investment strategy for poor ROI
    async fn adjust_investment_strategy(&self, poor_cycles: u32, _threshold: f64) -> Result<AdjustmentResult> {
        tracing::info!("💰 Adjusting investment strategy after {} poor cycles", poor_cycles);

        // Strategy: Shift to more conservative, higher-probability investments
        let strategy_changes = HashMap::from([
            ("risk_tolerance".to_string(), StrategyChange {
                parameter: "risk_tolerance".to_string(),
                old_value: json!("moderate"),
                new_value: json!("conservative"),
                reason: "Reduce risk after poor performance".to_string(),
            }),
            ("diversification".to_string(), StrategyChange {
                parameter: "diversification".to_string(),
                old_value: json!(0.6),
                new_value: json!(0.8),
                reason: "Increase diversification for stability".to_string(),
            }),
        ]);

        // Send adjustment message to investment agent
        self.bus.send_to_agent(
            "GOVERNANCE",
            "INV-LOCAL-001",
            "strategy_adjustment",
            json!({
                "adjustment_type": "investment_rebalancing",
                "new_parameters": {
                    "risk_tolerance": "conservative",
                    "diversification_ratio": 0.8,
                    "focus_sectors": ["established_fintech", "stable_foodtech"],
                    "max_single_investment": 0.15
                }
            })
        ).await?;

        Ok(AdjustmentResult {
            adjustment_id: uuid::Uuid::new_v4().to_string(),
            adjustment_type: AdjustmentType::InvestmentRebalancing,
            affected_agents: vec!["INV-LOCAL-001".to_string()],
            strategy_changes,
            expected_impact: ExpectedImpact {
                roi_improvement: 0.03,
                efficiency_gain: 0.05,
                risk_reduction: 0.15,
                measurement_timeline_days: 30,
            },
        })
    }

    /// Optimize underperforming agent
    async fn optimize_agent_performance(&self, agent_id: &str, current_score: f64) -> Result<AdjustmentResult> {
        tracing::info!("🤖 Optimizing performance for agent {} (current score: {:.2})", agent_id, current_score);

        let strategy_changes = HashMap::from([
            ("confidence_threshold".to_string(), StrategyChange {
                parameter: "confidence_threshold".to_string(),
                old_value: json!(0.7),
                new_value: json!(0.8),
                reason: "Increase confidence threshold for better decisions".to_string(),
            }),
            ("analysis_depth".to_string(), StrategyChange {
                parameter: "analysis_depth".to_string(),
                old_value: json!("standard"),
                new_value: json!("deep"),
                reason: "Use more thorough analysis methods".to_string(),
            }),
        ]);

        // Send optimization parameters to the agent
        self.bus.send_to_agent(
            "GOVERNANCE",
            agent_id,
            "performance_optimization",
            json!({
                "optimization_type": "decision_quality",
                "new_parameters": {
                    "confidence_threshold": 0.8,
                    "analysis_depth": "deep",
                    "validation_steps": ["market_check", "risk_assessment", "peer_review"],
                    "decision_timeout_ms": 5000
                }
            })
        ).await?;

        Ok(AdjustmentResult {
            adjustment_id: uuid::Uuid::new_v4().to_string(),
            adjustment_type: AdjustmentType::CoordinationTuning,
            affected_agents: vec![agent_id.to_string()],
            strategy_changes,
            expected_impact: ExpectedImpact {
                roi_improvement: 0.02,
                efficiency_gain: 0.15,
                risk_reduction: 0.08,
                measurement_timeline_days: 14,
            },
        })
    }

    /// Stabilize system performance
    async fn stabilize_system_performance(&self, variance: f64) -> Result<AdjustmentResult> {
        tracing::info!("⚖️ Stabilizing system performance (variance: {:.2})", variance);

        let strategy_changes = HashMap::from([
            ("coordination_timeout".to_string(), StrategyChange {
                parameter: "coordination_timeout".to_string(),
                old_value: json!(30),
                new_value: json!(60),
                reason: "Increase timeout to reduce coordination failures".to_string(),
            }),
            ("consensus_threshold".to_string(), StrategyChange {
                parameter: "consensus_threshold".to_string(),
                old_value: json!(0.6),
                new_value: json!(0.8),
                reason: "Require higher consensus for decisions".to_string(),
            }),
        ]);

        // Broadcast stabilization parameters to all agents
        self.bus.broadcast(
            "GOVERNANCE",
            "system_stabilization",
            MessageType::Command,
            json!({
                "stabilization_mode": "consensus_based",
                "parameters": {
                    "coordination_timeout_seconds": 60,
                    "consensus_threshold": 0.8,
                    "retry_attempts": 3,
                    "fallback_strategy": "conservative"
                }
            })
        ).await?;

        Ok(AdjustmentResult {
            adjustment_id: uuid::Uuid::new_v4().to_string(),
            adjustment_type: AdjustmentType::CoordinationTuning,
            affected_agents: vec!["ALL".to_string()],
            strategy_changes,
            expected_impact: ExpectedImpact {
                roi_improvement: 0.01,
                efficiency_gain: 0.10,
                risk_reduction: 0.20,
                measurement_timeline_days: 7,
            },
        })
    }

    /// Generic system optimization
    async fn perform_generic_optimization(&self) -> Result<AdjustmentResult> {
        tracing::info!("🔧 Performing generic system optimization");

        let strategy_changes = HashMap::from([
            ("optimization_mode".to_string(), StrategyChange {
                parameter: "optimization_mode".to_string(),
                old_value: json!("balanced"),
                new_value: json!("efficiency_focused"),
                reason: "Focus on efficiency improvements".to_string(),
            }),
        ]);

        // Send optimization signal to all agents
        self.bus.coordinate(
            "GOVERNANCE",
            "system_optimization",
            "optimize_performance",
            vec!["INV-LOCAL-001".to_string(), "BIZ-LOCAL-001".to_string(), 
                 "CFO-LOCAL-001".to_string(), "USER-LOCAL-001".to_string()]
        ).await?;

        Ok(AdjustmentResult {
            adjustment_id: uuid::Uuid::new_v4().to_string(),
            adjustment_type: AdjustmentType::CoordinationTuning,
            affected_agents: vec!["ALL".to_string()],
            strategy_changes,
            expected_impact: ExpectedImpact {
                roi_improvement: 0.02,
                efficiency_gain: 0.12,
                risk_reduction: 0.05,
                measurement_timeline_days: 21,
            },
        })
    }

    /// Send governance recommendations without auto-adjustment
    async fn send_governance_recommendations(&self, issues: Vec<GovernanceTrigger>) -> Result<()> {
        let recommendations = json!({
            "governance_recommendations": {
                "issues_identified": issues.len(),
                "recommendations": issues.iter().map(|issue| {
                    match issue {
                        GovernanceTrigger::ConsistentPoorROI { cycles, threshold } => {
                            json!({
                                "issue": "consistent_poor_roi",
                                "details": format!("{} cycles below {:.1}%", cycles, threshold * 100.0),
                                "recommendation": "Consider investment strategy rebalancing"
                            })
                        },
                        GovernanceTrigger::AgentUnderperformance { agent_id, score } => {
                            json!({
                                "issue": "agent_underperformance",
                                "details": format!("Agent {} score: {:.2}", agent_id, score),
                                "recommendation": "Review and optimize agent parameters"
                            })
                        },
                        GovernanceTrigger::PerformanceInstability { variance } => {
                            json!({
                                "issue": "performance_instability",
                                "details": format!("Variance: {:.2}", variance),
                                "recommendation": "Implement coordination stabilization"
                            })
                        },
                        _ => json!({
                            "issue": "generic",
                            "recommendation": "Review system configuration"
                        })
                    }
                }).collect::<Vec<_>>(),
                "timestamp": Utc::now()
            }
        });

        self.bus.broadcast(
            "GOVERNANCE",
            "system_recommendations",
            MessageType::Alert,
            recommendations
        ).await?;

        Ok(())
    }

    /// Update system-wide KPIs
    async fn update_system_kpis(&self) -> Result<()> {
        let mut tracker = self.performance_tracker.write().await;
        
        // Calculate efficiency score based on recent performance
        let comparison = self.state_manager.get_performance_comparison().await?;
        let avg_success_rate = comparison.values()
            .map(|p| p.success_rate)
            .sum::<f64>() / comparison.len().max(1) as f64;

        tracker.system_kpis.efficiency_score = avg_success_rate;
        
        // Update coordination quality based on message flow
        let bus_stats = self.bus.get_stats().await;
        tracker.system_kpis.coordination_quality = 
            (bus_stats.active_subscriptions as f64 / 20.0).min(1.0); // Normalize to max 20 subscriptions

        // Update decision consistency (simulated)
        tracker.system_kpis.decision_consistency = 
            comparison.values().map(|p| p.accuracy_score).sum::<f64>() / comparison.len().max(1) as f64;

        tracker.last_action_at = Utc::now();
        
        Ok(())
    }

    /// Get governance status and metrics
    pub async fn get_governance_status(&self) -> GovernanceStatus {
        let tracker = self.performance_tracker.read().await;
        let adjustment_history = self.adjustment_history.read().await;
        
        GovernanceStatus {
            system_kpis: tracker.system_kpis.clone(),
            consecutive_poor_cycles: tracker.consecutive_poor_cycles,
            last_action_at: tracker.last_action_at,
            total_adjustments: adjustment_history.len() as u32,
            recent_adjustments: adjustment_history.iter().rev().take(5).cloned().collect(),
            governance_health: tracker.system_kpis.efficiency_score * 0.4 + 
                              tracker.system_kpis.coordination_quality * 0.3 +
                              tracker.system_kpis.stability_score * 0.3,
        }
    }
}

/// Performance data collection structure
#[derive(Debug)]
struct PerformanceData {
    agent_performance: HashMap<String, crate::ai::agent_state::PerformanceMetrics>,
    cycle_history: Vec<CyclePerformance>,
    health_trend: Vec<f64>,
    communication_health: f64,
}

impl PerformanceData {
    fn new() -> Self {
        Self {
            agent_performance: HashMap::new(),
            cycle_history: Vec::new(),
            health_trend: Vec::new(),
            communication_health: 0.0,
        }
    }
    
    /// Calculate current system efficiency (0.0 - 1.0)
    fn calculate_efficiency(&self) -> f64 {
        if self.agent_performance.is_empty() {
            return 0.3; // Default for empty system
        }
        
        // Average success rate of all agents
        let total_success: f64 = self.agent_performance.values()
            .map(|metrics| metrics.success_rate)
            .sum();
        let avg_success = total_success / self.agent_performance.len() as f64;
        
        // Factor in communication health
        let communication_factor = self.communication_health * 0.3;
        
        // Factor in recent cycle performance
        let cycle_factor = if !self.cycle_history.is_empty() {
            let recent_cycles = self.cycle_history.iter().rev().take(3);
            let avg_roi: f64 = recent_cycles.map(|c| c.roi).sum::<f64>() / 3.0_f64.min(self.cycle_history.len() as f64);
            (avg_roi * 0.2).min(0.4) // Cap at 0.4 contribution
        } else {
            0.1
        };
        
        (avg_success * 0.5 + communication_factor + cycle_factor).min(1.0)
    }
    
    /// Calculate current ROI (Return on Investment)
    fn calculate_roi(&self) -> f64 {
        if self.cycle_history.is_empty() {
            return 0.05; // Default minimal ROI
        }
        
        // Average ROI from recent cycles (weighted toward more recent)
        let recent_cycles: Vec<&CyclePerformance> = self.cycle_history.iter().rev().take(5).collect();
        if recent_cycles.is_empty() {
            return 0.05;
        }
        
        let mut weighted_roi = 0.0;
        let mut weight_sum = 0.0;
        
        for (i, cycle) in recent_cycles.iter().enumerate() {
            let weight = (i + 1) as f64; // More recent cycles have higher weight
            weighted_roi += cycle.roi * weight;
            weight_sum += weight;
        }
        
        weighted_roi / weight_sum
    }
}

/// Result of a strategic adjustment
#[derive(Debug)]
struct AdjustmentResult {
    adjustment_id: String,
    adjustment_type: AdjustmentType,
    affected_agents: Vec<String>,
    strategy_changes: HashMap<String, StrategyChange>,
    expected_impact: ExpectedImpact,
}

/// Current governance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStatus {
    /// Current system KPIs
    pub system_kpis: SystemKPIs,
    /// Number of consecutive poor cycles
    pub consecutive_poor_cycles: u32,
    /// Last governance action timestamp
    pub last_action_at: DateTime<Utc>,
    /// Total adjustments made
    pub total_adjustments: u32,
    /// Recent adjustments
    pub recent_adjustments: Vec<StrategyAdjustment>,
    /// Overall governance health score
    pub governance_health: f64,
}

impl AIGovernanceLayer {
    /// 🧠 SELF-LEARNING: Automatically adjust strategy weights based on performance
    pub async fn auto_adjust_strategy_weights(&self, efficiency: f64, roi: f64) -> Result<Vec<ResourceReallocation>> {
        let mut reallocations = Vec::new();
        let mut weights = self.strategy_weights.write().await;
        
        tracing::info!("🧠 Auto-adjusting strategy weights - Efficiency: {:.2}, ROI: {:.2}", efficiency, roi);
        
        // Poor performance - reallocate resources
        if efficiency < 0.5 {
            tracing::warn!("⚠️ Low efficiency detected ({}), reallocating resources", efficiency);
            
            // Boost marketing if efficiency is low
            let marketing_boost = 0.2;
            reallocations.push(ResourceReallocation {
                from_strategy: "investment".to_string(),
                to_strategy: "marketing".to_string(),
                transfer_amount: marketing_boost,
                reason: format!("Low efficiency ({:.2}) - boost marketing to drive growth", efficiency),
                expected_improvement: 0.15,
            });
            
            // Reduce investment allocation if ROI is poor
            if roi < 0.1 {
                let investment_reduction = 0.1;
                reallocations.push(ResourceReallocation {
                    from_strategy: "investment".to_string(),
                    to_strategy: "business_dev".to_string(),
                    transfer_amount: investment_reduction,
                    reason: format!("Poor ROI ({:.2}) - reduce investment, focus on business development", roi),
                    expected_improvement: 0.08,
                });
                
                // Update weights
                weights.investment_weight = (weights.investment_weight - investment_reduction - marketing_boost).max(0.05);
                weights.marketing_weight = (weights.marketing_weight + marketing_boost).min(0.5);
                weights.business_dev_weight = (weights.business_dev_weight + investment_reduction).min(0.4);
            } else {
                // Update weights for marketing boost only
                weights.investment_weight = (weights.investment_weight - marketing_boost).max(0.1);
                weights.marketing_weight = (weights.marketing_weight + marketing_boost).min(0.5);
            }
        }
        // Good performance - conservative optimization
        else if efficiency > 0.8 && roi > 0.2 {
            tracing::info!("✅ Excellent performance - making conservative optimizations");
            
            // Small boost to proven strategies
            let conservative_boost = 0.05;
            reallocations.push(ResourceReallocation {
                from_strategy: "risk_management".to_string(),
                to_strategy: "user_acquisition".to_string(),
                transfer_amount: conservative_boost,
                reason: "High performance - expand user acquisition while maintaining success".to_string(),
                expected_improvement: 0.03,
            });
            
            weights.risk_management_weight = (weights.risk_management_weight - conservative_boost).max(0.05);
            weights.user_acquisition_weight = (weights.user_acquisition_weight + conservative_boost).min(0.25);
        }
        
        // Update metadata
        weights.updated_at = Utc::now();
        weights.confidence_score = (weights.confidence_score + 0.1).min(1.0); // Increase confidence with each adjustment
        
        // Learn from this adjustment
        self.update_learning_data(efficiency, roi, &reallocations).await?;
        
        tracing::info!("🔄 Strategy weights updated: Marketing={:.2}, Investment={:.2}, Business={:.2}", 
            weights.marketing_weight, weights.investment_weight, weights.business_dev_weight);
        
        Ok(reallocations)
    }
    
    /// Update learning data based on recent performance and adjustments
    async fn update_learning_data(&self, efficiency: f64, roi: f64, reallocations: &[ResourceReallocation]) -> Result<()> {
        let mut learning = self.learning_data.write().await;
        
        // Update strategy effectiveness scores
        for reallocation in reallocations {
            let current_score = learning.strategy_effectiveness
                .get(&reallocation.to_strategy)
                .unwrap_or(&0.5);
            
            // Increase effectiveness score for boosted strategies
            let new_score = (current_score + 0.1 * reallocation.expected_improvement).min(1.0);
            learning.strategy_effectiveness.insert(reallocation.to_strategy.clone(), new_score);
            
            // Slightly decrease for reduced strategies (but not too much)
            let reduced_score = learning.strategy_effectiveness
                .get(&reallocation.from_strategy)
                .unwrap_or(&0.5);
            let new_reduced_score = (reduced_score - 0.05 * reallocation.transfer_amount).max(0.1);
            learning.strategy_effectiveness.insert(reallocation.from_strategy.clone(), new_reduced_score);
        }
        
        // Update performance predictors
        learning.performance_predictors.insert("last_efficiency".to_string(), efficiency);
        learning.performance_predictors.insert("last_roi".to_string(), roi);
        
        // Discover patterns if we have enough data
        if learning.strategy_effectiveness.len() >= 3 {
            self.discover_allocation_patterns(&mut learning, efficiency, roi).await;
        }
        
        learning.last_learning_update = Utc::now();
        
        tracing::info!("📊 Learning data updated - {} effectiveness scores tracked", 
            learning.strategy_effectiveness.len());
        
        Ok(())
    }
    
    /// Discover successful allocation patterns from historical data
    async fn discover_allocation_patterns(&self, learning: &mut LearningData, current_efficiency: f64, current_roi: f64) {
        let weights = self.strategy_weights.read().await;
        
        // If current performance is good, save this as a successful pattern
        if current_efficiency > 0.7 && current_roi > 0.15 {
            let pattern = AllocationPattern {
                name: format!("High_Performance_Pattern_{}", Utc::now().timestamp()),
                weights: weights.clone(),
                optimal_conditions: vec![
                    format!("efficiency_>{}", current_efficiency),
                    format!("roi_>{}", current_roi),
                ],
                success_rate: 0.8, // Start with optimistic rate
                average_roi: current_roi,
            };
            
            // Only keep top 5 patterns to avoid memory bloat
            learning.optimal_patterns.push(pattern);
            if learning.optimal_patterns.len() > 5 {
                learning.optimal_patterns.remove(0); // Remove oldest
            }
            
            tracing::info!("🎯 Discovered new successful allocation pattern");
        }
    }
    
    /// Get current strategy weights for inspection
    pub async fn get_strategy_weights(&self) -> StrategyWeights {
        self.strategy_weights.read().await.clone()
    }
    
    /// Get learning insights for monitoring
    pub async fn get_learning_insights(&self) -> LearningData {
        self.learning_data.read().await.clone()
    }
    
    /// Apply a discovered allocation pattern
    pub async fn apply_allocation_pattern(&self, pattern_name: &str) -> Result<bool> {
        let learning = self.learning_data.read().await;
        
        if let Some(pattern) = learning.optimal_patterns.iter().find(|p| p.name == pattern_name) {
            let mut weights = self.strategy_weights.write().await;
            *weights = pattern.weights.clone();
            weights.updated_at = Utc::now();
            
            tracing::info!("🎯 Applied allocation pattern: {} (Success rate: {:.1}%)", 
                pattern_name, pattern.success_rate * 100.0);
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Apply resource reallocations to the system
    async fn apply_resource_reallocations(&self, reallocations: Vec<ResourceReallocation>) -> Result<()> {
        for reallocation in reallocations {
            tracing::info!("💰 Reallocating {:.1}% from {} to {} - {}", 
                reallocation.transfer_amount * 100.0,
                reallocation.from_strategy,
                reallocation.to_strategy,
                reallocation.reason
            );
            
            // Send reallocation commands to relevant agents via shared bus
            let message = json!({
                "reallocation_type": "strategy_weight_adjustment",
                "from_strategy": reallocation.from_strategy,
                "to_strategy": reallocation.to_strategy,
                "transfer_amount": reallocation.transfer_amount,
                "reason": reallocation.reason,
                "expected_improvement": reallocation.expected_improvement,
                "timestamp": Utc::now(),
                "governance_source": "auto_learning"
            });
            
            // Broadcast to all agents so they can adjust their behavior
            self.bus.broadcast(
                "AI_GOVERNANCE", 
                "strategy_reallocation", 
                MessageType::Command, 
                message
            ).await?;
            
            tracing::info!("📡 Reallocation broadcast sent to all agents");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_governance_creation() {
        let bus = Arc::new(SharedBus::new().await.unwrap());
        let temp_dir = tempdir().unwrap();
        let state_manager = Arc::new(
            AgentStateManager::new(temp_dir.path().to_str().unwrap()).await.unwrap()
        );
        
        let governance = AIGovernanceLayer::new(bus, state_manager, None).await.unwrap();
        let status = governance.get_governance_status().await;
        
        assert!(status.governance_health > 0.0);
        assert_eq!(status.consecutive_poor_cycles, 0);
    }
}