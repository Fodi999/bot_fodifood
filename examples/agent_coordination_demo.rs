//! 🔄 Agent Coordination Demo
//! 
//! Demonstrates multi-agent coordination with response chains:
//! System → Investor → Business → CFO workflow

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use serde_json::json;

use fodifood_bot::ai::SharedBus;
use fodifood_bot::ai::shared_bus::{
    CoordinationResult, CoordinationStatus, WorkflowStepResult,
    MessageType, BusMessage
};

/// Demo agent that processes coordination messages and responds
pub struct DemoAgent {
    pub id: String,
    pub agent_type: String,
    pub bus: Arc<SharedBus>,
}

impl DemoAgent {
    pub async fn new(id: &str, agent_type: &str, bus: Arc<SharedBus>) -> Self {
        Self {
            id: id.to_string(),
            agent_type: agent_type.to_string(),
            bus,
        }
    }

    /// Start listening for messages and responding
    pub async fn start_listening(&self) -> anyhow::Result<()> {
        // Subscribe to relevant topics
        let topics = match self.agent_type.as_str() {
            "investor" => vec!["coordination".to_string(), "workflow".to_string(), "market_analysis".to_string()],
            "business" => vec!["coordination".to_string(), "workflow".to_string(), "investment_report".to_string()],
            "cfo" => vec!["coordination".to_string(), "workflow".to_string(), "business_forecast".to_string()],
            _ => vec!["coordination".to_string()],
        };

        let mut receiver = self.bus.subscribe(&self.id, topics).await?;
        println!("🎧 {} listening for messages...", self.id);

        // Message processing loop
        while let Ok(message) = receiver.recv().await {
            self.process_message(message).await?;
        }

        Ok(())
    }

    /// Process incoming messages and generate responses
    async fn process_message(&self, message: BusMessage) -> anyhow::Result<()> {
        println!("📨 {} received: {} on topic '{}'", 
                 self.id, message.message_type, message.topic);

        let topic = message.topic.as_str();
        let msg_type = &message.message_type;
        
        match (topic, msg_type) {
            ("coordination", MessageType::Coordination) => {
                self.handle_coordination(&message).await?;
            },
            ("workflow", MessageType::Event) => {
                self.handle_workflow(&message).await?;
            },
            ("market_analysis", MessageType::Request) => {
                self.handle_market_analysis(&message).await?;
            },
            ("investment_report", MessageType::Response) => {
                self.handle_investment_report(&message).await?;
            },
            ("business_forecast", MessageType::Response) => {
                self.handle_business_forecast(&message).await?;
            },
            _ => {
                println!("🤷 {} ignoring message type {:?} on topic {}", 
                         self.id, msg_type, topic);
            }
        }

        Ok(())
    }

    /// Handle coordination requests
    async fn handle_coordination(&self, message: &BusMessage) -> anyhow::Result<()> {
        if let Some(task_id) = message.payload.get("task_id").and_then(|v| v.as_str()) {
            if let Some(action) = message.payload.get("action").and_then(|v| v.as_str()) {
                println!("🎯 {} executing coordination task: {} - {}", self.id, task_id, action);
                
                // Simulate processing time
                sleep(Duration::from_millis(500)).await;
                
                // Generate result based on agent type
                let result_data = match self.agent_type.as_str() {
                    "investor" => json!({
                        "analysis": "Market conditions favorable for investment",
                        "risk_score": 0.3,
                        "recommendation": "Proceed with investment analysis",
                        "confidence": 0.85
                    }),
                    "business" => json!({
                        "market_outlook": "Positive growth expected in Q4",
                        "competitive_analysis": "Strong position in market",
                        "growth_potential": 0.75,
                        "strategic_recommendations": ["Expand marketing", "Optimize operations"]
                    }),
                    "cfo" => json!({
                        "financial_impact": "Positive ROI expected",
                        "budget_allocation": 250000,
                        "dividend_adjustment": 0.15,
                        "approval_status": "Approved"
                    }),
                    _ => json!({ "status": "acknowledged" })
                };

                let result = CoordinationResult {
                    task_id: task_id.to_string(),
                    agent_id: self.id.clone(),
                    status: CoordinationStatus::Success,
                    result: result_data.clone(),
                    processing_time_ms: 500,
                    next_steps: Some(vec![
                        format!("Results ready for next phase"),
                        format!("Awaiting coordinator decision")
                    ]),
                    completed_at: chrono::Utc::now(),
                };

                // Send result back
                self.bus.send_coordination_result(&self.id, task_id, result).await?;
                println!("✅ {} completed coordination task: {}", self.id, task_id);

                // Trigger next step in workflow if this is the investor
                if self.agent_type == "investor" && action == "analyze_investment_opportunity" {
                    sleep(Duration::from_millis(200)).await;
                    self.bus.trigger_workflow(
                        &self.id,
                        "market_analysis_workflow",
                        "business_forecast",
                        json!({
                            "investment_analysis": result_data,
                            "next_agent": "BIZ-LOCAL-001"
                        })
                    ).await?;
                    println!("🔄 {} triggered workflow: business_forecast", self.id);
                }
            }
        }
        Ok(())
    }

    /// Handle workflow step execution
    async fn handle_workflow(&self, message: &BusMessage) -> anyhow::Result<()> {
        if let Some(workflow_id) = message.payload.get("workflow_id").and_then(|v| v.as_str()) {
            if let Some(step) = message.payload.get("step").and_then(|v| v.as_str()) {
                println!("⚙️ {} executing workflow step: {} - {}", self.id, workflow_id, step);
                
                // Simulate processing
                sleep(Duration::from_millis(800)).await;
                
                let (output, next_step) = match (self.agent_type.as_str(), step) {
                    ("business", "business_forecast") => (
                        json!({
                            "forecast": "25% revenue growth expected",
                            "market_expansion": "3 new regions identified",
                            "risk_factors": ["Competition", "Market volatility"],
                            "recommended_budget": 180000
                        }),
                        Some("financial_planning".to_string())
                    ),
                    ("cfo", "financial_planning") => (
                        json!({
                            "budget_approved": 180000,
                            "dividend_increase": 0.12,
                            "cash_flow_projection": "Positive for next 12 months",
                            "investment_decision": "Approved for execution"
                        }),
                        None
                    ),
                    _ => (json!({"status": "processed"}), None)
                };

                let result = WorkflowStepResult {
                    workflow_id: workflow_id.to_string(),
                    step: step.to_string(),
                    agent_id: self.id.clone(),
                    status: CoordinationStatus::Success,
                    output: output.clone(),
                    next_step: next_step.clone(),
                    executed_at: chrono::Utc::now(),
                };

                self.bus.complete_workflow_step(result).await?;
                println!("✅ {} completed workflow step: {}", self.id, step);

                // Trigger next step if exists
                if let Some(next) = next_step {
                    if self.agent_type == "business" && next == "financial_planning" {
                        sleep(Duration::from_millis(200)).await;
                        self.bus.trigger_workflow(
                            &self.id,
                            workflow_id,
                            &next,
                            json!({
                                "business_forecast": output,
                                "next_agent": "CFO-LOCAL-001"
                            })
                        ).await?;
                        println!("🔄 {} triggered next workflow step: {}", self.id, next);
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle market analysis requests
    async fn handle_market_analysis(&self, _message: &BusMessage) -> anyhow::Result<()> {
        println!("📊 {} performing market analysis...", self.id);
        sleep(Duration::from_millis(1000)).await;
        
        self.bus.broadcast(
            &self.id,
            "investment_report",
            MessageType::Response,
            json!({
                "analysis_type": "market_opportunity",
                "sectors": ["fintech", "foodtech", "proptech"],
                "investment_score": 8.2,
                "recommendation": "Strong buy signal"
            })
        ).await?;
        
        println!("📈 {} published investment report", self.id);
        Ok(())
    }

    /// Handle investment report processing
    async fn handle_investment_report(&self, _message: &BusMessage) -> anyhow::Result<()> {
        if self.agent_type == "business" {
            println!("🏢 {} processing investment report...", self.id);
            sleep(Duration::from_millis(600)).await;
            
            self.bus.broadcast(
                &self.id,
                "business_forecast",
                MessageType::Response,
                json!({
                    "forecast_type": "growth_projection",
                    "revenue_increase": "28%",
                    "market_share_target": "15%",
                    "timeline": "Q1-Q3 2026"
                })
            ).await?;
            
            println!("🏢 {} published business forecast", self.id);
        }
        Ok(())
    }

    /// Handle business forecast processing
    async fn handle_business_forecast(&self, _message: &BusMessage) -> anyhow::Result<()> {
        if self.agent_type == "cfo" {
            println!("💰 {} processing business forecast...", self.id);
            sleep(Duration::from_millis(400)).await;
            
            self.bus.send_alert(
                &self.id,
                "system_alerts",
                "Dividend increase approved: +12% based on growth forecast",
                6
            ).await?;
            
            println!("💰 {} completed financial planning", self.id);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Agent Coordination Demo");
    
    // Create shared bus
    let bus = Arc::new(SharedBus::new().await?);
    
    // Create demo agents
    let investor = Arc::new(DemoAgent::new("INV-DEMO-001", "investor", bus.clone()).await);
    let business = Arc::new(DemoAgent::new("BIZ-DEMO-001", "business", bus.clone()).await);
    let cfo = Arc::new(DemoAgent::new("CFO-DEMO-001", "cfo", bus.clone()).await);
    
    // Start agents in parallel
    let investor_task = {
        let agent = investor.clone();
        tokio::spawn(async move { agent.start_listening().await })
    };
    
    let business_task = {
        let agent = business.clone();
        tokio::spawn(async move { agent.start_listening().await })
    };
    
    let cfo_task = {
        let agent = cfo.clone();
        tokio::spawn(async move { agent.start_listening().await })
    };
    
    // Wait for agents to start
    sleep(Duration::from_millis(1500)).await;
    
    println!("\n🎬 Demo Scenario 1: Simple Coordination");
    bus.coordinate(
        "SYSTEM",
        "market-analysis-demo",
        "analyze_investment_opportunity",
        vec!["INV-DEMO-001".to_string(), "BIZ-DEMO-001".to_string(), "CFO-DEMO-001".to_string()]
    ).await?;
    
    // Wait for coordination to complete and workflow to propagate
    sleep(Duration::from_secs(5)).await;
    
    println!("\n🎬 Demo Scenario 2: Chain Reaction");
    bus.send_to_agent("SYSTEM", "INV-DEMO-001", "market_analysis", json!({
        "request_type": "sector_analysis",
        "sectors": ["fintech", "foodtech"]
    })).await?;
    
    // Wait for chain to complete
    sleep(Duration::from_secs(6)).await;
    
    println!("\n📊 Final Bus Statistics:");
    let stats = bus.get_stats().await;
    println!("Total messages: {}", stats.total_messages);
    println!("Messages per topic: {:#?}", stats.messages_per_topic);
    println!("Active subscriptions: {}", stats.active_subscriptions);
    
    // Cleanup
    investor_task.abort();
    business_task.abort();
    cfo_task.abort();
    
    println!("\n✅ Demo completed!");
    Ok(())
}