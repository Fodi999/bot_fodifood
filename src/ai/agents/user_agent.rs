//! 👤 User Agent - AI Agent for User Interaction and Personalization
//! 
//! Specialized AI agent focused on user experience, personalized interactions,
//! conversation history, and preference learning.

use super::memory_store::{MemoryStore, MemoryQuery, MemorySortBy};
use crate::ai::agent_manager::{AIEntityAgent, AgentType, AgentState, AgentStatus, AgentConfig};
use crate::ai::persistent_memory::PersistentMemory;
use crate::ai::thinker::Thinker;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// User-focused AI agent with personalization capabilities
pub struct UserAgent {
    /// Agent unique identifier
    id: String,
    /// Memory store for persistent data
    memory_store: Arc<MemoryStore>,
    /// AI thinking capabilities
    thinker: Thinker,
    /// User profile and preferences
    user_profile: Arc<RwLock<UserProfile>>,
    /// Conversation history and context
    conversation_context: Arc<RwLock<ConversationContext>>,
    /// Agent configuration
    config: AgentConfig,
    /// Agent state information
    state: Arc<RwLock<AgentState>>,
    /// User-specific knowledge and insights
    knowledge: Arc<RwLock<UserKnowledge>>,
}

/// Comprehensive user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User identifier
    pub user_id: String,
    /// Display name
    pub name: String,
    /// User preferences
    pub preferences: UserPreferences,
    /// Communication style
    pub communication_style: CommunicationStyle,
    /// Interests and topics
    pub interests: Vec<UserInterest>,
    /// Interaction patterns
    pub interaction_patterns: InteractionPatterns,
    /// Personal context
    pub personal_context: PersonalContext,
    /// Learning preferences
    pub learning_preferences: LearningPreferences,
}

/// User preferences and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred response length
    pub response_length: ResponseLength,
    /// Communication formality
    pub formality_level: FormalityLevel,
    /// Topics of interest
    pub preferred_topics: Vec<String>,
    /// Topics to avoid
    pub avoided_topics: Vec<String>,
    /// Notification preferences
    pub notifications: NotificationPreferences,
    /// Language preference
    pub language: String,
    /// Timezone
    pub timezone: String,
    /// Response timing preferences
    pub response_timing: ResponseTiming,
}

/// Response length preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseLength {
    Brief,      // 1-2 sentences
    Normal,     // 1-2 paragraphs
    Detailed,   // 3-4 paragraphs
    Comprehensive, // Full analysis
}

/// Formality level in communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormalityLevel {
    Casual,      // Informal, friendly
    Professional, // Business-like
    Formal,      // Respectful, structured
    Adaptive,    // Match user's style
}

/// Communication style profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// Typical emotional tone
    pub emotional_tone: EmotionalTone,
    /// Preferred interaction style
    pub interaction_style: InteractionStyle,
    /// Information processing preference
    pub information_style: InformationStyle,
    /// Decision-making style
    pub decision_style: DecisionStyle,
    /// Humor appreciation
    pub humor_level: HumorLevel,
}

/// Emotional tone in conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalTone {
    Optimistic,
    Neutral,
    Analytical,
    Empathetic,
    Enthusiastic,
    Cautious,
}

/// Interaction style preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionStyle {
    Direct,      // Straight to the point
    Conversational, // Natural dialogue
    Structured,  // Organized, methodical
    Exploratory, // Question-driven
}

/// Information processing style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InformationStyle {
    Visual,      // Prefers charts, diagrams
    Textual,     // Prefers detailed text
    Bullet,      // Prefers bullet points
    Narrative,   // Prefers stories/examples
}

/// Decision-making style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionStyle {
    DataDriven,  // Wants numbers and facts
    Intuitive,   // Trusts gut feelings
    Collaborative, // Seeks input from others
    Quick,       // Prefers fast decisions
    Deliberate,  // Takes time to decide
}

/// Humor appreciation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HumorLevel {
    None,        // Prefers serious tone
    Subtle,      // Light, professional humor
    Moderate,    // Regular humor is fine
    High,        // Enjoys jokes and wit
}

/// User interest with engagement tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInterest {
    /// Interest topic
    pub topic: String,
    /// Interest level (0.0-1.0)
    pub level: f64,
    /// How often discussed
    pub frequency: u32,
    /// Last discussed
    pub last_discussed: chrono::DateTime<chrono::Utc>,
    /// Engagement score
    pub engagement_score: f64,
    /// Related subtopics
    pub subtopics: Vec<String>,
}

/// Interaction patterns and behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPatterns {
    /// Typical session duration (minutes)
    pub avg_session_duration: u32,
    /// Most active times of day
    pub active_hours: Vec<u8>,
    /// Preferred days of week
    pub active_days: Vec<String>,
    /// Question types asked
    pub question_patterns: HashMap<String, u32>,
    /// Response satisfaction rates
    pub satisfaction_scores: Vec<f64>,
    /// Engagement trends
    pub engagement_trend: EngagementTrend,
}

/// Engagement trend over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngagementTrend {
    Increasing,
    Stable,
    Decreasing,
    Variable,
}

/// Personal context for better understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalContext {
    /// Professional role/industry
    pub profession: Option<String>,
    /// Current goals or projects
    pub current_goals: Vec<String>,
    /// Challenges or pain points
    pub challenges: Vec<String>,
    /// Achievements or successes
    pub achievements: Vec<String>,
    /// Personal values
    pub values: Vec<String>,
    /// Life stage/situation
    pub life_stage: Option<String>,
}

/// Learning and growth preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPreferences {
    /// Preferred learning style
    pub learning_style: LearningStyle,
    /// Complexity preference
    pub complexity_level: ComplexityLevel,
    /// Example preference
    pub example_preference: ExamplePreference,
    /// Follow-up question tendency
    pub follow_up_tendency: f64,
    /// Exploration vs. efficiency
    pub exploration_vs_efficiency: f64, // 0.0 = efficiency, 1.0 = exploration
}

/// Learning style preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningStyle {
    StepByStep,  // Sequential, methodical
    Overview,    // Big picture first
    Practical,   // Hands-on examples
    Theoretical, // Concepts and principles
    Mixed,       // Combination approach
}

/// Complexity level preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Beginner,    // Simple explanations
    Intermediate, // Moderate detail
    Advanced,    // Full complexity
    Expert,      // Technical depth
    Adaptive,    // Match to context
}

/// Example preference in explanations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExamplePreference {
    None,        // Just theory
    Few,         // 1-2 examples
    Many,        // Multiple examples
    RealWorld,   // Practical examples
    Personal,    // Relevant to user
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Enable notifications
    pub enabled: bool,
    /// Notification frequency
    pub frequency: NotificationFrequency,
    /// Types of notifications
    pub types: Vec<NotificationType>,
    /// Quiet hours
    pub quiet_hours: (u8, u8), // Start and end hour
}

/// Notification frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Immediate,
    Hourly,
    Daily,
    Weekly,
    None,
}

/// Types of notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Updates,
    Reminders,
    Insights,
    Questions,
    Social,
}

/// Response timing preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTiming {
    /// Expected response time
    pub expected_speed: ResponseSpeed,
    /// Patience level for detailed responses
    pub patience_level: f64,
    /// Preference for thinking time indicators
    pub show_thinking: bool,
}

/// Response speed expectation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseSpeed {
    Instant,     // Immediate response
    Quick,       // Within seconds
    Normal,      // Within minute
    Patient,     // Can wait for quality
}

/// Conversation context and history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Current session ID
    pub session_id: String,
    /// Current topic being discussed
    pub current_topic: Option<String>,
    /// Context from recent messages
    pub recent_context: Vec<ContextMessage>,
    /// Conversation mood/tone
    pub conversation_mood: ConversationMood,
    /// User's current state/situation
    pub user_state: UserState,
    /// Session statistics
    pub session_stats: SessionStats,
}

/// Context message for conversation continuity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    /// Message content
    pub content: String,
    /// Message type
    pub message_type: MessageType,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Importance score
    pub importance: f64,
    /// Topic tags
    pub topics: Vec<String>,
}

/// Message type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Question,
    Statement,
    Request,
    Response,
    Clarification,
    Feedback,
}

/// Current conversation mood
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationMood {
    Friendly,
    Professional,
    Curious,
    Focused,
    Relaxed,
    Urgent,
    Confused,
    Satisfied,
}

/// User's current state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserState {
    Exploring,    // Looking around
    Focused,      // Working on specific task
    Learning,     // Trying to understand
    Deciding,     // Making a choice
    Frustrated,   // Having difficulty
    Satisfied,    // Got what they needed
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    /// Messages exchanged
    pub message_count: u32,
    /// Session start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Topics covered
    pub topics_covered: Vec<String>,
    /// User satisfaction (if provided)
    pub satisfaction: Option<f64>,
    /// Follow-up questions asked
    pub follow_up_count: u32,
}

/// User-specific knowledge and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserKnowledge {
    /// Learned preferences over time
    pub learned_preferences: Vec<LearnedPreference>,
    /// Conversation patterns
    pub conversation_patterns: Vec<ConversationPattern>,
    /// Success factors for interactions
    pub success_factors: Vec<SuccessFactor>,
    /// Areas of expertise shown by user
    pub user_expertise: Vec<UserExpertise>,
    /// Common questions or topics
    pub frequent_topics: HashMap<String, TopicStats>,
}

/// Learned preference from interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPreference {
    /// Preference description
    pub preference: String,
    /// Confidence level
    pub confidence: f64,
    /// Times observed
    pub observations: u32,
    /// Last confirmed
    pub last_confirmed: chrono::DateTime<chrono::Utc>,
}

/// Conversation pattern observed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPattern {
    /// Pattern description
    pub pattern: String,
    /// How often seen
    pub frequency: u32,
    /// Contexts where it appears
    pub contexts: Vec<String>,
    /// User satisfaction when pattern followed
    pub satisfaction_correlation: f64,
}

/// Success factor for interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessFactor {
    /// Factor description
    pub factor: String,
    /// Impact on success
    pub impact_score: f64,
    /// Times validated
    pub validations: u32,
    /// Specific examples
    pub examples: Vec<String>,
}

/// User expertise in specific areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExpertise {
    /// Area of expertise
    pub area: String,
    /// Expertise level (0.0-1.0)
    pub level: f64,
    /// Evidence/examples
    pub evidence: Vec<String>,
    /// Times demonstrated
    pub demonstrations: u32,
}

/// Topic statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicStats {
    /// Times discussed
    pub frequency: u32,
    /// Average engagement
    pub avg_engagement: f64,
    /// User satisfaction on topic
    pub satisfaction: f64,
    /// Last discussed
    pub last_discussed: chrono::DateTime<chrono::Utc>,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            user_id: "default_user".to_string(),
            name: "User".to_string(),
            preferences: UserPreferences::default(),
            communication_style: CommunicationStyle::default(),
            interests: Vec::new(),
            interaction_patterns: InteractionPatterns::default(),
            personal_context: PersonalContext::default(),
            learning_preferences: LearningPreferences::default(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            response_length: ResponseLength::Normal,
            formality_level: FormalityLevel::Adaptive,
            preferred_topics: Vec::new(),
            avoided_topics: Vec::new(),
            notifications: NotificationPreferences::default(),
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            response_timing: ResponseTiming::default(),
        }
    }
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        Self {
            emotional_tone: EmotionalTone::Neutral,
            interaction_style: InteractionStyle::Conversational,
            information_style: InformationStyle::Textual,
            decision_style: DecisionStyle::DataDriven,
            humor_level: HumorLevel::Subtle,
        }
    }
}

impl Default for InteractionPatterns {
    fn default() -> Self {
        Self {
            avg_session_duration: 15,
            active_hours: vec![9, 10, 11, 14, 15, 16, 20, 21],
            active_days: vec!["Monday".to_string(), "Tuesday".to_string(), "Wednesday".to_string()],
            question_patterns: HashMap::new(),
            satisfaction_scores: Vec::new(),
            engagement_trend: EngagementTrend::Stable,
        }
    }
}

impl Default for PersonalContext {
    fn default() -> Self {
        Self {
            profession: None,
            current_goals: Vec::new(),
            challenges: Vec::new(),
            achievements: Vec::new(),
            values: Vec::new(),
            life_stage: None,
        }
    }
}

impl Default for LearningPreferences {
    fn default() -> Self {
        Self {
            learning_style: LearningStyle::Mixed,
            complexity_level: ComplexityLevel::Adaptive,
            example_preference: ExamplePreference::Few,
            follow_up_tendency: 0.5,
            exploration_vs_efficiency: 0.6,
        }
    }
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency: NotificationFrequency::Daily,
            types: vec![NotificationType::Updates, NotificationType::Insights],
            quiet_hours: (22, 7), // 10 PM to 7 AM
        }
    }
}

impl Default for ResponseTiming {
    fn default() -> Self {
        Self {
            expected_speed: ResponseSpeed::Normal,
            patience_level: 0.7,
            show_thinking: true,
        }
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self {
            session_id: format!("session_{}", chrono::Utc::now().timestamp()),
            current_topic: None,
            recent_context: Vec::new(),
            conversation_mood: ConversationMood::Friendly,
            user_state: UserState::Exploring,
            session_stats: SessionStats {
                message_count: 0,
                started_at: chrono::Utc::now(),
                topics_covered: Vec::new(),
                satisfaction: None,
                follow_up_count: 0,
            },
        }
    }
}

impl Default for UserKnowledge {
    fn default() -> Self {
        Self {
            learned_preferences: Vec::new(),
            conversation_patterns: Vec::new(),
            success_factors: Vec::new(),
            user_expertise: Vec::new(),
            frequent_topics: HashMap::new(),
        }
    }
}

impl UserAgent {
    /// Create new user agent
    pub async fn new(id: &str, persistent_memory: Arc<PersistentMemory>) -> Result<Self> {
        let memory_store = Arc::new(MemoryStore::new(persistent_memory).await?);
        let thinker = Thinker;
        let user_profile = Arc::new(RwLock::new(UserProfile::default()));
        let conversation_context = Arc::new(RwLock::new(ConversationContext::default()));
        let config = AgentConfig::default();
        
        let state = Arc::new(RwLock::new(AgentState {
            id: id.to_string(),
            agent_type: AgentType::User,
            created_at: chrono::Utc::now(),
            last_active: chrono::Utc::now(),
            interaction_count: 0,
            memory_size: 0,
            status: AgentStatus::Active,
            config_version: 1,
        }));
        
        let knowledge = Arc::new(RwLock::new(UserKnowledge::default()));

        let agent = Self {
            id: id.to_string(),
            memory_store,
            thinker,
            user_profile,
            conversation_context,
            config,
            state,
            knowledge,
        };

        // Initialize with basic user interaction knowledge
        agent.initialize_memories().await?;

        Ok(agent)
    }

    /// Initialize agent with basic user interaction knowledge
    async fn initialize_memories(&self) -> Result<()> {
        self.memory_store.store(
            &self.id,
            "system",
            "role",
            "I am a user-focused AI agent that provides personalized, helpful interactions"
        ).await?;

        self.memory_store.store(
            &self.id,
            "capabilities",
            "personalization",
            "I learn user preferences, communication style, and interests for better interactions"
        ).await?;

        self.memory_store.store(
            &self.id,
            "approach",
            "user_centric",
            "I adapt my responses to match user's preferred communication style and information needs"
        ).await?;

        Ok(())
    }

    /// Process user interaction with personalization
    async fn process_user_interaction(&mut self, input: &str) -> Result<String> {
        // Update conversation context
        self.update_conversation_context(input).await?;
        
        // Analyze user input for preferences and patterns
        self.analyze_user_input(input).await?;
        
        // Generate personalized response
        let response = self.generate_personalized_response(input).await?;
        
        // Update user knowledge and patterns
        self.update_user_knowledge(input, &response).await?;
        
        Ok(response)
    }

    /// Update conversation context with new message
    async fn update_conversation_context(&self, input: &str) -> Result<()> {
        let mut context = self.conversation_context.write().await;
        
        // Add message to recent context
        context.recent_context.push(ContextMessage {
            content: input.to_string(),
            message_type: self.classify_message_type(input),
            timestamp: chrono::Utc::now(),
            importance: self.calculate_message_importance(input),
            topics: self.extract_topics(input),
        });

        // Keep only recent messages (last 10)
        if context.recent_context.len() > 10 {
            context.recent_context.remove(0);
        }

        // Update session stats
        context.session_stats.message_count += 1;
        
        // Update current topic if detected
        if let Some(topic) = self.detect_current_topic(input) {
            context.current_topic = Some(topic.clone());
            if !context.session_stats.topics_covered.contains(&topic) {
                context.session_stats.topics_covered.push(topic);
            }
        }

        // Update user state based on message
        context.user_state = self.infer_user_state(input, &context.user_state);
        
        // Update conversation mood
        context.conversation_mood = self.detect_conversation_mood(input);

        Ok(())
    }

    /// Classify message type
    fn classify_message_type(&self, input: &str) -> MessageType {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("?") || input_lower.starts_with("what") || 
           input_lower.starts_with("how") || input_lower.starts_with("why") ||
           input_lower.starts_with("when") || input_lower.starts_with("where") {
            MessageType::Question
        } else if input_lower.contains("please") || input_lower.contains("can you") ||
                  input_lower.starts_with("show me") || input_lower.starts_with("help") {
            MessageType::Request
        } else if input_lower.contains("i think") || input_lower.contains("in my opinion") ||
                  input_lower.contains("i believe") {
            MessageType::Statement
        } else if input_lower.contains("what do you mean") || input_lower.contains("clarify") {
            MessageType::Clarification
        } else if input_lower.contains("good") || input_lower.contains("bad") ||
                  input_lower.contains("thanks") || input_lower.contains("helpful") {
            MessageType::Feedback
        } else {
            MessageType::Statement
        }
    }

    /// Calculate message importance
    fn calculate_message_importance(&self, input: &str) -> f64 {
        let mut importance: f64 = 0.5; // Base importance
        
        // Increase importance for questions
        if input.contains("?") {
            importance += 0.2;
        }
        
        // Increase for requests
        if input.to_lowercase().contains("please") || input.to_lowercase().contains("help") {
            importance += 0.2;
        }
        
        // Increase for feedback
        if input.to_lowercase().contains("good") || input.to_lowercase().contains("bad") {
            importance += 0.3;
        }
        
        // Increase for personal information
        if input.to_lowercase().contains("i am") || input.to_lowercase().contains("my") {
            importance += 0.2;
        }
        
        importance.min(1.0)
    }

    /// Extract topics from message
    fn extract_topics(&self, input: &str) -> Vec<String> {
        let mut topics = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Business topics
        if input_lower.contains("business") || input_lower.contains("revenue") || 
           input_lower.contains("profit") || input_lower.contains("customer") {
            topics.push("business".to_string());
        }
        
        // Investment topics
        if input_lower.contains("investment") || input_lower.contains("portfolio") ||
           input_lower.contains("roi") || input_lower.contains("dividend") {
            topics.push("investment".to_string());
        }
        
        // Technology topics
        if input_lower.contains("technology") || input_lower.contains("ai") ||
           input_lower.contains("software") || input_lower.contains("blockchain") {
            topics.push("technology".to_string());
        }
        
        // Personal topics
        if input_lower.contains("personal") || input_lower.contains("goal") ||
           input_lower.contains("preference") || input_lower.contains("like") {
            topics.push("personal".to_string());
        }
        
        topics
    }

    /// Detect current topic of conversation
    fn detect_current_topic(&self, input: &str) -> Option<String> {
        let topics = self.extract_topics(input);
        topics.first().cloned()
    }

    /// Infer user state from message
    fn infer_user_state(&self, input: &str, current_state: &UserState) -> UserState {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("confused") || input_lower.contains("don't understand") {
            UserState::Learning
        } else if input_lower.contains("thanks") || input_lower.contains("perfect") ||
                  input_lower.contains("exactly") {
            UserState::Satisfied
        } else if input_lower.contains("frustrated") || input_lower.contains("difficult") {
            UserState::Frustrated
        } else if input_lower.contains("learn") || input_lower.contains("understand") ||
                  input_lower.contains("explain") {
            UserState::Learning
        } else if input_lower.contains("decide") || input_lower.contains("choose") ||
                  input_lower.contains("should i") {
            UserState::Deciding
        } else if input_lower.contains("focus") || input_lower.contains("working on") {
            UserState::Focused
        } else {
            match current_state {
                UserState::Satisfied => UserState::Exploring, // Reset after satisfaction
                _ => current_state.clone(),
            }
        }
    }

    /// Detect conversation mood
    fn detect_conversation_mood(&self, input: &str) -> ConversationMood {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("urgent") || input_lower.contains("quickly") ||
           input_lower.contains("asap") {
            ConversationMood::Urgent
        } else if input_lower.contains("professional") || input_lower.contains("business") {
            ConversationMood::Professional
        } else if input_lower.contains("curious") || input_lower.contains("wonder") ||
                  input_lower.contains("?") {
            ConversationMood::Curious
        } else if input_lower.contains("focus") || input_lower.contains("specific") {
            ConversationMood::Focused
        } else if input_lower.contains("confused") || input_lower.contains("help") {
            ConversationMood::Confused
        } else if input_lower.contains("thanks") || input_lower.contains("great") {
            ConversationMood::Satisfied
        } else if input_lower.contains("casual") || input_lower.contains("chat") {
            ConversationMood::Relaxed
        } else {
            ConversationMood::Friendly
        }
    }

    /// Analyze user input for preferences and patterns
    async fn analyze_user_input(&mut self, input: &str) -> Result<()> {
        let input_lower = input.to_lowercase();
        
        // Detect response length preference
        if input_lower.contains("brief") || input_lower.contains("short") ||
           input_lower.contains("quickly") {
            self.update_preference("response_length", "brief").await?;
        } else if input_lower.contains("detail") || input_lower.contains("comprehensive") ||
                  input_lower.contains("explain fully") {
            self.update_preference("response_length", "detailed").await?;
        }
        
        // Detect formality preference
        if input_lower.contains("formal") || input_lower.contains("professional") {
            self.update_preference("formality", "formal").await?;
        } else if input_lower.contains("casual") || input_lower.contains("informal") {
            self.update_preference("formality", "casual").await?;
        }
        
        // Detect information style preference
        if input_lower.contains("bullet") || input_lower.contains("list") ||
           input_lower.contains("points") {
            self.update_preference("information_style", "bullet").await?;
        } else if input_lower.contains("example") || input_lower.contains("show me") {
            self.update_preference("information_style", "examples").await?;
        }
        
        // Detect learning style
        if input_lower.contains("step by step") || input_lower.contains("gradually") {
            self.update_preference("learning_style", "step_by_step").await?;
        } else if input_lower.contains("overview") || input_lower.contains("big picture") {
            self.update_preference("learning_style", "overview").await?;
        }

        Ok(())
    }

    /// Update user preference
    async fn update_preference(&self, preference_type: &str, value: &str) -> Result<()> {
        self.memory_store.store(
            &self.id,
            "preferences",
            preference_type,
            value
        ).await?;

        // Update knowledge base
        let mut knowledge = self.knowledge.write().await;
        
        // Find existing preference or create new one
        if let Some(pref) = knowledge.learned_preferences.iter_mut()
            .find(|p| p.preference.contains(preference_type)) {
            pref.observations += 1;
            pref.confidence = (pref.confidence + 0.1).min(1.0);
            pref.last_confirmed = chrono::Utc::now();
        } else {
            knowledge.learned_preferences.push(LearnedPreference {
                preference: format!("{}: {}", preference_type, value),
                confidence: 0.7,
                observations: 1,
                last_confirmed: chrono::Utc::now(),
            });
        }

        Ok(())
    }

    /// Generate personalized response based on user profile
    async fn generate_personalized_response(&mut self, input: &str) -> Result<String> {
        let profile = self.user_profile.read().await;
        let context = self.conversation_context.read().await;
        
        // Build personalized context for AI thinking
        let personalization_context = format!(
            "User Profile Context:\n\
            - Communication Style: {:?}\n\
            - Response Length Preference: {:?}\n\
            - Formality Level: {:?}\n\
            - Information Style: {:?}\n\
            - Current Mood: {:?}\n\
            - User State: {:?}\n\
            - Current Topic: {:?}\n\
            - Session Message Count: {}\n\n\
            Recent Context: {}\n\n\
            User Input: {}",
            profile.communication_style.interaction_style,
            profile.preferences.response_length,
            profile.preferences.formality_level,
            profile.communication_style.information_style,
            context.conversation_mood,
            context.user_state,
            context.current_topic.as_deref().unwrap_or("general"),
            context.session_stats.message_count,
            context.recent_context.iter()
                .take(3)
                .map(|msg| format!("- {}", msg.content))
                .collect::<Vec<_>>()
                .join("\n"),
            input
        );

        // Generate response with personalization
        let mut response = Thinker::think(&personalization_context).await?;
        
        // Apply style adjustments based on preferences
        response = self.apply_style_preferences(&response, &profile).await;
        
        Ok(response)
    }

    /// Apply style preferences to response
    async fn apply_style_preferences(&self, response: &str, profile: &UserProfile) -> String {
        let mut styled_response = response.to_string();
        
        // Apply response length preference
        match profile.preferences.response_length {
            ResponseLength::Brief => {
                // Keep only first paragraph for brief responses
                if let Some(first_para) = styled_response.split("\n\n").next() {
                    styled_response = first_para.to_string();
                }
            }
            ResponseLength::Detailed => {
                // Add more detail if response is too short
                if styled_response.len() < 200 {
                    styled_response.push_str("\n\nWould you like me to expand on any particular aspect of this topic?");
                }
            }
            _ => {} // Normal and Comprehensive use original length
        }
        
        // Apply formality level
        match profile.preferences.formality_level {
            FormalityLevel::Casual => {
                styled_response = styled_response.replace("However,", "But");
                styled_response = styled_response.replace("Therefore,", "So");
                styled_response = styled_response.replace("Furthermore,", "Also,");
            }
            FormalityLevel::Formal => {
                styled_response = styled_response.replace("can't", "cannot");
                styled_response = styled_response.replace("won't", "will not");
                styled_response = styled_response.replace("don't", "do not");
            }
            _ => {} // Professional and Adaptive use original style
        }
        
        // Apply information style
        match profile.communication_style.information_style {
            InformationStyle::Bullet => {
                // Convert to bullet points if not already formatted
                if !styled_response.contains("•") && !styled_response.contains("-") {
                    let sentences: Vec<&str> = styled_response.split(". ").collect();
                    if sentences.len() > 2 {
                        styled_response = sentences.iter()
                            .enumerate()
                            .map(|(i, sentence)| {
                                if i < sentences.len() - 1 {
                                    format!("• {}", sentence.trim())
                                } else {
                                    format!("• {}", sentence.trim_end_matches('.'))
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                    }
                }
            }
            _ => {} // Other styles use original format
        }
        
        styled_response
    }

    /// Update user knowledge based on interaction
    async fn update_user_knowledge(&mut self, input: &str, response: &str) -> Result<()> {
        let mut knowledge = self.knowledge.write().await;
        
        // Update topic frequency
        let topics = self.extract_topics(input);
        for topic in topics {
            let stats = knowledge.frequent_topics.entry(topic).or_insert(TopicStats {
                frequency: 0,
                avg_engagement: 0.7,
                satisfaction: 0.8,
                last_discussed: chrono::Utc::now(),
            });
            stats.frequency += 1;
            stats.last_discussed = chrono::Utc::now();
        }
        
        // Detect conversation patterns
        let pattern = self.detect_conversation_pattern(input, response);
        if let Some(pattern_desc) = pattern {
            if let Some(existing_pattern) = knowledge.conversation_patterns.iter_mut()
                .find(|p| p.pattern == pattern_desc) {
                existing_pattern.frequency += 1;
            } else {
                knowledge.conversation_patterns.push(ConversationPattern {
                    pattern: pattern_desc,
                    frequency: 1,
                    contexts: vec![input.to_string()],
                    satisfaction_correlation: 0.8,
                });
            }
        }

        Ok(())
    }

    /// Detect conversation patterns
    fn detect_conversation_pattern(&self, input: &str, _response: &str) -> Option<String> {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("help") && input_lower.contains("?") {
            Some("help_seeking_question".to_string())
        } else if input_lower.contains("what") && input_lower.contains("difference") {
            Some("comparison_question".to_string())
        } else if input_lower.contains("how") && input_lower.contains("?") {
            Some("how_to_question".to_string())
        } else if input_lower.contains("explain") || input_lower.contains("understand") {
            Some("explanation_request".to_string())
        } else if input_lower.contains("example") || input_lower.contains("show me") {
            Some("example_request".to_string())
        } else {
            None
        }
    }

    /// Update user profile with new information
    pub async fn update_profile(&mut self, name: Option<String>, interests: Option<Vec<String>>) -> Result<()> {
        let mut profile = self.user_profile.write().await;
        
        if let Some(new_name) = name {
            profile.name = new_name.clone();
            self.memory_store.store(
                &self.id,
                "profile",
                "name",
                &new_name
            ).await?;
        }
        
        if let Some(new_interests) = interests {
            for interest in new_interests {
                let user_interest = UserInterest {
                    topic: interest.clone(),
                    level: 0.8,
                    frequency: 1,
                    last_discussed: chrono::Utc::now(),
                    engagement_score: 0.7,
                    subtopics: Vec::new(),
                };
                profile.interests.push(user_interest);
                
                self.memory_store.store(
                    &self.id,
                    "interests",
                    "topic",
                    &interest
                ).await?;
            }
        }

        Ok(())
    }

    /// Get personalized user summary
    pub async fn get_user_summary(&self) -> String {
        let profile = self.user_profile.read().await;
        let context = self.conversation_context.read().await;
        let knowledge = self.knowledge.read().await;
        
        format!(
            "👤 User Profile Summary:\n\n\
            🎯 User: {}\n\
            💬 Communication Style: {:?}\n\
            📊 Response Preference: {:?}\n\
            🎨 Information Style: {:?}\n\n\
            📈 Interaction Stats:\n\
            • Total Messages: {}\n\
            • Current Mood: {:?}\n\
            • User State: {:?}\n\
            • Topics Covered: {}\n\n\
            🧠 Learned Preferences: {}\n\
            🔄 Conversation Patterns: {}\n\
            📚 Frequent Topics: {}\n\n\
            💡 Personalization Confidence: High\n\
            🎪 Adaptation Status: Active",
            profile.name,
            profile.communication_style.interaction_style,
            profile.preferences.response_length,
            profile.communication_style.information_style,
            context.session_stats.message_count,
            context.conversation_mood,
            context.user_state,
            context.session_stats.topics_covered.join(", "),
            knowledge.learned_preferences.len(),
            knowledge.conversation_patterns.len(),
            knowledge.frequent_topics.len()
        )
    }
}

impl AIEntityAgent for UserAgent {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> AgentType {
        AgentType::User
    }

    fn think(&mut self, input: &str) -> Result<String> {
        // Update last active time
        if let Ok(mut state) = self.state.try_write() {
            state.last_active = chrono::Utc::now();
            state.interaction_count += 1;
        }

        // Process personalized user interaction
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.process_user_interaction(input).await
            })
        })
    }

    fn recall(&self, query: Option<&str>) -> String {
        let memories = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                if let Some(q) = query {
                    self.memory_store.search(MemoryQuery {
                        agent_id: Some(self.id.clone()),
                        category: None,
                        search_text: Some(q.to_string()),
                        tags: Vec::new(),
                        min_importance: None,
                        limit: Some(10),
                        sort_by: MemorySortBy::Relevance,
                    }).await.unwrap_or_default()
                } else {
                    self.memory_store.get_agent_memories(&self.id).await.unwrap_or_default()
                }
            })
        });

        if memories.is_empty() {
            format!("👤 User Agent {} Memory: Learning your preferences and interaction patterns...", self.id)
        } else {
            let memory_summary: Vec<String> = memories.iter()
                .take(5)
                .map(|m| format!("• {}: {}", m.key, m.value))
                .collect();
            
            format!("👤 User Agent {} Memory:\n{}", self.id, memory_summary.join("\n"))
        }
    }

    fn memorize(&mut self, key: &str, value: &str) -> Result<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.memory_store.store(&self.id, "manual", key, value).await
            })
        })?;
        Ok(())
    }

    fn get_state_summary(&self) -> AgentState {
        self.state.blocking_read().clone()
    }

    fn receive_message(&mut self, from_agent: &str, message: &str) -> Result<Option<String>> {
        let response = format!(
            "📨 Message from {}: {}\n\
            👤 Thank you for sharing this information. I'll use it to provide \
            more personalized and helpful responses in our future interactions.",
            from_agent, message
        );

        self.memorize(
            &format!("message_from_{}", from_agent),
            &format!("{}: {}", from_agent, message)
        )?;

        Ok(Some(response))
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec![
            "Personalized Communication".to_string(),
            "User Preference Learning".to_string(),
            "Conversation Context Management".to_string(),
            "Adaptive Response Styling".to_string(),
            "Interest Pattern Recognition".to_string(),
            "Communication Style Matching".to_string(),
            "User State Detection".to_string(),
            "Interaction Pattern Analysis".to_string(),
        ]
    }

    fn update_config(&mut self, config: AgentConfig) -> Result<()> {
        self.config = config;
        
        if let Ok(mut state) = self.state.try_write() {
            state.config_version += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_agent_creation() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_user.db").unwrap());
        let agent = UserAgent::new("USER-TEST", persistent_memory).await.unwrap();
        
        assert_eq!(agent.get_id(), "USER-TEST");
        assert!(matches!(agent.get_type(), AgentType::User));
    }

    #[tokio::test]
    async fn test_personalized_interaction() {
        let persistent_memory = Arc::new(PersistentMemory::new("test_user2.db").unwrap());
        let mut agent = UserAgent::new("USER-TEST2", persistent_memory).await.unwrap();
        
        let response = agent.think("I like brief, casual responses please").unwrap();
        assert!(response.len() > 0);
        
        // Test that preference was learned
        let profile = agent.user_profile.read().await;
        // Preferences should be updated in the knowledge base
        assert!(!agent.knowledge.read().await.learned_preferences.is_empty());
    }
}