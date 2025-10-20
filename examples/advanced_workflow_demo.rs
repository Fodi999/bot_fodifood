//! 🔄 Advanced Agent Workflow Chain Demo
//! 
//! Demonstrates complete agent coordination workflow:
//! System Request → Investment Analysis → Business Planning → Financial Decision

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use serde_json::json;

use fodifood_bot::ai::SharedBus;
use fodifood_bot::ai::shared_bus::{
    CoordinationResult, CoordinationStatus, WorkflowStepResult,
    MessageType, BusMessage
};

/// Advanced workflow orchestrator that demonstrates real-world agent coordination
pub struct WorkflowOrchestrator {
    pub bus: Arc<SharedBus>,
    pub workflow_state: std::sync::Arc<tokio::sync::RwLock<WorkflowState>>,
}

#[derive(Clone, Debug)]
pub struct WorkflowState {
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub results: std::collections::HashMap<String, serde_json::Value>,
    pub workflow_id: String,
}

impl WorkflowOrchestrator {
    pub async fn new(bus: Arc<SharedBus>) -> Self {
        Self {
            bus,
            workflow_state: Arc::new(tokio::sync::RwLock::new(WorkflowState {
                current_step: "initial".to_string(),
                completed_steps: vec![],
                results: std::collections::HashMap::new(),
                workflow_id: "WF-001".to_string(),
            })),
        }
    }

    /// Start the complete investment decision workflow
    pub async fn start_investment_workflow(&self) -> anyhow::Result<()> {
        println!("🚀 Starting Investment Decision Workflow...");
        
        // Phase 1: Initial Market Analysis Request
        println!("\n📊 Phase 1: Requesting Market Analysis from Investment Agent");
        self.bus.send_to_agent(
            "ORCHESTRATOR",
            "INV-DEMO-001",
            "market_analysis",
            json!({
                "workflow_id": "investment-decision-2024",
                "analysis_type": "comprehensive_market_scan",
                "sectors": ["fintech", "foodtech", "proptech"],
                "deadline": "2024-12-31",
                "budget_range": [100000, 500000]
            })
        ).await?;

        // Wait for investment analysis
        sleep(Duration::from_secs(2)).await;

        // Phase 2: Business Strategy Development  
        println!("\n🏢 Phase 2: Business Strategy Development");
        self.bus.trigger_workflow(
            "ORCHESTRATOR",
            "business-strategy-workflow", 
            "strategic_planning",
            json!({
                "base_investment": {
                    "sectors": ["fintech", "foodtech"],
                    "confidence": 0.85,
                    "expected_roi": "28%"
                },
                "target_agent": "BIZ-DEMO-001"
            })
        ).await?;

        // Wait for business strategy
        sleep(Duration::from_secs(2)).await;

        // Phase 3: Financial Planning & Approval
        println!("\n💰 Phase 3: Financial Planning & CFO Approval");
        self.bus.trigger_workflow(
            "ORCHESTRATOR",
            "financial-approval-workflow",
            "budget_allocation",
            json!({
                "business_plan": {
                    "revenue_projection": "25% growth",
                    "market_expansion": "3 new regions",
                    "budget_request": 250000
                },
                "target_agent": "CFO-DEMO-001"
            })
        ).await?;

        // Wait for final approval
        sleep(Duration::from_secs(2)).await;

        // Phase 4: Coordination Summary
        println!("\n🎯 Phase 4: Final Coordination & Decision");
        self.bus.coordinate(
            "ORCHESTRATOR",
            "final-investment-decision",
            "approve_investment_plan", 
            vec!["INV-DEMO-001".to_string(), "BIZ-DEMO-001".to_string(), "CFO-DEMO-001".to_string()]
        ).await?;

        Ok(())
    }

    /// Print workflow status
    pub async fn print_status(&self) {
        let state = self.workflow_state.read().await;
        println!("\n📋 Workflow Status:");
        println!("   • Current Step: {}", state.current_step);
        println!("   • Completed: {} steps", state.completed_steps.len());
        println!("   • Results Collected: {} agents", state.results.len());
    }
}

/// Enhanced demo agent with workflow awareness
pub struct WorkflowAwareAgent {
    pub id: String,
    pub agent_type: String,
    pub bus: Arc<SharedBus>,
    pub orchestrator: Arc<WorkflowOrchestrator>,
}

impl WorkflowAwareAgent {
    pub async fn new(
        id: &str, 
        agent_type: &str, 
        bus: Arc<SharedBus>,
        orchestrator: Arc<WorkflowOrchestrator>
    ) -> Self {
        Self {
            id: id.to_string(),
            agent_type: agent_type.to_string(),
            bus,
            orchestrator,
        }
    }

    /// Start listening with enhanced workflow processing
    pub async fn start_enhanced_listening(&self) -> anyhow::Result<()> {
        let topics = match self.agent_type.as_str() {
            "investor" => vec!["coordination".to_string(), "workflow".to_string(), "market_analysis".to_string()],
            "business" => vec!["coordination".to_string(), "workflow".to_string(), "investment_report".to_string()],
            "cfo" => vec!["coordination".to_string(), "workflow".to_string(), "business_forecast".to_string()],
            _ => vec!["coordination".to_string()],
        };

        let mut receiver = self.bus.subscribe(&self.id, topics).await?;
        println!("🎧 {} listening for workflow messages...", self.id);

        while let Ok(message) = receiver.recv().await {
            self.process_workflow_message(message).await?;
        }

        Ok(())
    }

    /// Enhanced message processing with workflow context
    async fn process_workflow_message(&self, message: BusMessage) -> anyhow::Result<()> {
        println!("📨 {} received: {:?} on topic '{}'", 
                 self.id, message.message_type, message.topic);

        match (message.topic.as_str(), &message.message_type) {
            ("market_analysis", MessageType::Request) => {
                self.handle_market_analysis_request(&message).await?;
            },
            ("workflow", MessageType::Event) => {
                self.handle_workflow_step(&message).await?;
            },
            ("coordination", MessageType::Coordination) => {
                self.handle_final_coordination(&message).await?;
            },
            _ => {
                println!("🔄 {} processing standard message on topic {}", self.id, message.topic);
            }
        }

        Ok(())
    }

    /// Handle market analysis requests with detailed responses
    async fn handle_market_analysis_request(&self, message: &BusMessage) -> anyhow::Result<()> {
        if self.agent_type == "investor" {
            println!("📊 {} performing comprehensive market analysis...", self.id);
            
            if let Some(workflow_id) = message.payload.get("workflow_id").and_then(|v| v.as_str()) {
                sleep(Duration::from_millis(1200)).await;
                
                // Generate detailed investment analysis
                let analysis_result = json!({
                    "market_analysis": {
                        "fintech_score": 8.7,
                        "foodtech_score": 9.2,
                        "proptech_score": 7.8,
                        "overall_sentiment": "bullish",
                        "risk_assessment": "moderate-low"
                    },
                    "investment_recommendation": {
                        "primary_sector": "foodtech",
                        "allocation": {
                            "foodtech": 60,
                            "fintech": 30, 
                            "proptech": 10
                        },
                        "expected_roi": "28-35%",
                        "timeline": "12-18 months"
                    },
                    "next_steps": ["business_strategy", "market_validation", "financial_planning"]
                });

                // Send comprehensive report to business agent
                self.bus.broadcast(
                    &self.id,
                    "investment_report",
                    MessageType::Response,
                    json!({
                        "workflow_id": workflow_id,
                        "report_type": "comprehensive_analysis",
                        "analysis": analysis_result,
                        "confidence": 0.92,
                        "next_agent": "BIZ-DEMO-001"
                    })
                ).await?;

                println!("📈 {} completed comprehensive market analysis", self.id);
            }
        }
        Ok(())
    }

    /// Handle workflow steps with chain progression
    async fn handle_workflow_step(&self, message: &BusMessage) -> anyhow::Result<()> {
        if let Some(step) = message.payload.get("step").and_then(|v| v.as_str()) {
            if let Some(workflow_id) = message.payload.get("workflow_id").and_then(|v| v.as_str()) {
                
                match (self.agent_type.as_str(), step) {
                    ("business", "strategic_planning") => {
                        println!("🏢 {} developing strategic business plan...", self.id);
                        sleep(Duration::from_millis(1500)).await;
                        
                        let business_strategy = json!({
                            "strategy": {
                                "market_entry": "agile_expansion",
                                "target_demographics": ["urban_millennials", "health_conscious_families"],
                                "competitive_advantage": "ai_powered_personalization",
                                "revenue_streams": ["subscription", "marketplace_fees", "premium_services"]
                            },
                            "growth_plan": {
                                "q1_2024": "pilot_program_3_cities",
                                "q2_2024": "scale_to_10_cities", 
                                "q3_2024": "national_expansion",
                                "q4_2024": "international_pilot"
                            },
                            "resource_requirements": {
                                "initial_capital": 250000,
                                "team_expansion": 15,
                                "technology_stack": ["ai_engine", "mobile_app", "logistics_platform"]
                            }
                        });

                        let result = WorkflowStepResult {
                            workflow_id: workflow_id.to_string(),
                            step: step.to_string(),
                            agent_id: self.id.clone(),
                            status: CoordinationStatus::Success,
                            output: business_strategy.clone(),
                            next_step: Some("budget_allocation".to_string()),
                            executed_at: chrono::Utc::now(),
                        };

                        self.bus.complete_workflow_step(result).await?;
                        
                        // Trigger financial planning
                        self.bus.broadcast(
                            &self.id,
                            "business_forecast",
                            MessageType::Response,
                            json!({
                                "workflow_id": workflow_id,
                                "business_plan": business_strategy,
                                "next_agent": "CFO-DEMO-001"
                            })
                        ).await?;

                        println!("✅ {} completed strategic planning", self.id);
                    },
                    ("cfo", "budget_allocation") => {
                        println!("💰 {} reviewing budget allocation and approvals...", self.id);
                        sleep(Duration::from_millis(1000)).await;
                        
                        let financial_decision = json!({
                            "budget_approval": {
                                "total_approved": 250000,
                                "allocation": {
                                    "technology": 100000,
                                    "marketing": 75000,
                                    "operations": 50000,
                                    "reserves": 25000
                                },
                                "approval_status": "approved",
                                "conditions": ["monthly_reporting", "milestone_reviews"]
                            },
                            "financial_projections": {
                                "break_even": "month_8",
                                "positive_cash_flow": "month_12",
                                "projected_revenue_y1": 850000,
                                "projected_profit_y1": 180000
                            },
                            "risk_mitigation": {
                                "cash_reserves": "3_month_runway",
                                "insurance_coverage": "comprehensive",
                                "legal_structure": "llc_setup"
                            }
                        });

                        let result = WorkflowStepResult {
                            workflow_id: workflow_id.to_string(),
                            step: step.to_string(),
                            agent_id: self.id.clone(),
                            status: CoordinationStatus::Success,
                            output: financial_decision,
                            next_step: None, // Final step
                            executed_at: chrono::Utc::now(),
                        };

                        self.bus.complete_workflow_step(result).await?;
                        println!("✅ {} approved budget and financial plan", self.id);
                    },
                    _ => {
                        println!("🔄 {} processing workflow step: {}", self.id, step);
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle final coordination with comprehensive summary
    async fn handle_final_coordination(&self, message: &BusMessage) -> anyhow::Result<()> {
        if let Some(task_id) = message.payload.get("task_id").and_then(|v| v.as_str()) {
            if let Some(action) = message.payload.get("action").and_then(|v| v.as_str()) {
                if action == "approve_investment_plan" {
                    println!("🎯 {} providing final recommendation for investment plan", self.id);
                    sleep(Duration::from_millis(800)).await;
                    
                    let final_recommendation = match self.agent_type.as_str() {
                        "investor" => json!({
                            "recommendation": "STRONG_BUY",
                            "confidence": 0.95,
                            "key_factors": ["strong_market_demand", "innovative_approach", "experienced_team"],
                            "investment_grade": "A+",
                            "timeline_confidence": "high"
                        }),
                        "business" => json!({
                            "recommendation": "PROCEED_WITH_EXECUTION",
                            "readiness_score": 0.88,
                            "strategic_alignment": "excellent",
                            "competitive_position": "strong",
                            "execution_confidence": "high"
                        }),
                        "cfo" => json!({
                            "recommendation": "FINANCIALLY_APPROVED",
                            "fiscal_responsibility": "excellent",
                            "roi_projection": "exceeds_expectations",
                            "risk_profile": "acceptable",
                            "cashflow_confidence": "strong"
                        }),
                        _ => json!({"status": "acknowledged"})
                    };

                    let result = CoordinationResult {
                        task_id: task_id.to_string(),
                        agent_id: self.id.clone(),
                        status: CoordinationStatus::Success,
                        result: final_recommendation,
                        processing_time_ms: 800,
                        next_steps: Some(vec![
                            "Investment plan approved by all agents".to_string(),
                            "Ready for execution phase".to_string()
                        ]),
                        completed_at: chrono::Utc::now(),
                    };

                    self.bus.send_coordination_result(&self.id, task_id, result).await?;
                    println!("✅ {} completed final coordination", self.id);
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Advanced Agent Workflow Demo");
    println!("   📊 Investment Analysis → 🏢 Business Strategy → 💰 Financial Planning");
    
    // Initialize shared bus
    let bus = Arc::new(SharedBus::new().await?);
    
    // Create workflow orchestrator
    let orchestrator = Arc::new(WorkflowOrchestrator::new(bus.clone()).await);
    
    // Create workflow-aware agents
    let investor = Arc::new(WorkflowAwareAgent::new(
        "INV-DEMO-001", 
        "investor", 
        bus.clone(),
        orchestrator.clone()
    ).await);
    
    let business = Arc::new(WorkflowAwareAgent::new(
        "BIZ-DEMO-001", 
        "business", 
        bus.clone(),
        orchestrator.clone()
    ).await);
    
    let cfo = Arc::new(WorkflowAwareAgent::new(
        "CFO-DEMO-001", 
        "cfo", 
        bus.clone(),
        orchestrator.clone()
    ).await);
    
    // Start agents in parallel
    let investor_task = {
        let agent = investor.clone();
        tokio::spawn(async move { agent.start_enhanced_listening().await })
    };
    
    let business_task = {
        let agent = business.clone();
        tokio::spawn(async move { agent.start_enhanced_listening().await })
    };
    
    let cfo_task = {
        let agent = cfo.clone();
        tokio::spawn(async move { agent.start_enhanced_listening().await })
    };
    
    // Wait for agents to initialize
    sleep(Duration::from_millis(1000)).await;
    
    // Execute the complete workflow
    orchestrator.start_investment_workflow().await?;
    
    // Wait for all steps to complete
    sleep(Duration::from_secs(8)).await;
    
    // Print final status and statistics
    orchestrator.print_status().await;
    
    println!("\n📊 Final Communication Bus Statistics:");
    let stats = bus.get_stats().await;
    println!("   • Total messages processed: {}", stats.total_messages);
    println!("   • Messages by topic: {:#?}", stats.messages_per_topic);
    println!("   • Active subscriptions: {}", stats.active_subscriptions);
    
    println!("\n🎉 Advanced Workflow Demo Completed Successfully!");
    println!("   ✅ Investment Analysis: Complete");
    println!("   ✅ Business Strategy: Complete");  
    println!("   ✅ Financial Planning: Complete");
    println!("   ✅ Final Coordination: Complete");
    
    // Cleanup
    investor_task.abort();
    business_task.abort();
    cfo_task.abort();
    
    Ok(())
}