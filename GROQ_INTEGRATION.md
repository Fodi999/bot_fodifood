# 🧠 Groq AI Integration Guide

## ✅ Successfully Integrated!

Groq Llama 3.1 теперь полностью интегрирован в FodiFood AI как ядро когнитивной системы.

---

## 📁 Structure

```
src/ai/
├── core/
│   ├── mod.rs          # Core exports
│   └── groq.rs         # Groq API client ✅ NEW
├── thinker.rs          # Enhanced with Groq ✅ UPDATED
├── intent_handler.rs   # Can use Groq
├── analysis.rs         # Can use Groq
└── admin_assistant.rs  # Can use Groq
```

---

## 🔑 Setup

### 1. Add GROQ_API_KEY

Add to `.env` or `Secrets.toml`:

```toml
# Secrets.toml
[default]
GROQ_API_KEY = "your_groq_api_key_here"
```

Or set environment variable:

```bash
export GROQ_API_KEY="your_groq_api_key_here"
```

### 2. Get API Key

1. Go to https://console.groq.com
2. Sign up/Login
3. Generate API key
4. Copy to Secrets.toml

---

## 🚀 Usage Examples

### 1. Basic Thinking (Llama 3.1 70B)

```rust
use crate::ai::thinker::Thinker;

// Simple query
let response = Thinker::think("What are the best pizza toppings?").await?;
println!("🧠 {}", response);
```

### 2. Fast Thinking (Llama 3.1 8B Instant)

```rust
// For quick, simple responses
let quick_answer = Thinker::think_fast("Say hello in 5 words").await?;
println!("⚡ {}", quick_answer);
```

### 3. Business Analysis

```rust
let data = "Revenue: 1.2M SOL, Orders: 342, Top dish: Pizza Margherita";
let analysis = Thinker::analyze_business(data).await?;
println!("📊 {}", analysis);
```

### 4. Personalized Recommendations

```rust
let context = "голодный, хочу что-то острое";
let prefs = Some("люблю морепродукты");

let recommendation = Thinker::get_ai_recommendation(context, prefs).await?;
println!("🎯 {}", recommendation);
```

### 5. Direct Groq API Call

```rust
use crate::ai::core::{query_groq, GroqConfig, GroqModel};

// Simple query
let response = query_groq("Explain blockchain in one sentence").await?;

// With custom config
let config = GroqConfig {
    model: GroqModel::Llama8B,  // Faster model
    temperature: 0.5,            // More focused
    max_tokens: 500,
    top_p: 0.9,
};

let response = query_groq_with_config("Summarize this...", &config).await?;
```

---

## 🧩 Integration Points

### In `intent_handler.rs`

```rust
use crate::ai::thinker::Thinker;

pub async fn handle_intent(
    intent: &str, 
    context: &str
) -> Result<String> {
    let prompt = format!(
        "User intent: {}. Context: {}. Provide helpful response.",
        intent, context
    );
    
    Thinker::think(&prompt).await
}
```

### In `analysis.rs`

```rust
use crate::ai::thinker::Thinker;

pub async fn analyze_sales(data: &SalesData) -> Result<String> {
    let summary = format!(
        "Total revenue: {}, Orders: {}, Avg check: {}",
        data.revenue, data.orders, data.avg_check
    );
    
    Thinker::analyze_business(&summary).await
}
```

### In `admin_assistant.rs`

```rust
use crate::ai::thinker::Thinker;

impl AdminAssistant {
    pub async fn get_insights(&self, query: &str) -> Result<String> {
        // Fetch real data
        let stats = self.fetch_stats().await?;
        
        // Use AI for analysis
        let analysis = Thinker::analyze_business(&stats).await?;
        
        Ok(analysis)
    }
}
```

---

## 🎯 Available Models

### Llama 3.1 70B Versatile
- **Use for**: Complex reasoning, business analysis, creative content
- **Speed**: ~1-2s response time
- **Context**: 8K tokens

```rust
let config = GroqConfig {
    model: GroqModel::Llama70B,
    ..Default::default()
};
```

### Llama 3.1 8B Instant
- **Use for**: Quick responses, simple queries, real-time chat
- **Speed**: < 500ms response time  
- **Context**: 8K tokens

```rust
let config = GroqConfig {
    model: GroqModel::Llama8B,
    ..Default::default()
};
```

### Mixtral 8x7B
- **Use for**: Balanced speed/quality
- **Speed**: ~1s response time
- **Context**: 32K tokens (best for long documents)

```rust
let config = GroqConfig {
    model: GroqModel::Mixtral,
    ..Default::default()
};
```

---

## 🔧 Configuration Options

```rust
pub struct GroqConfig {
    pub model: GroqModel,        // Model to use
    pub temperature: f32,        // 0.0 = focused, 1.0 = creative
    pub max_tokens: u32,         // Max response length
    pub top_p: f32,              // Nucleus sampling (0.9 default)
}
```

**Temperature Guide:**
- `0.0-0.3`: Factual, deterministic (analysis, data extraction)
- `0.4-0.7`: Balanced (general chat, Q&A)
- `0.8-1.0`: Creative (recommendations, storytelling)

---

## 📊 Full Workflow Example

```rust
use crate::ai::thinker::Thinker;
use crate::ai::intent_handler::IntentClassifier;

pub async fn process_user_message(user_id: &str, message: &str) -> Result<String> {
    // 1. Classify intent (local, fast)
    let intent = IntentClassifier::classify(message);
    
    // 2. Detect mood (local, fast)
    let mood = Thinker::detect_mood(message);
    
    // 3. For complex queries, use Groq
    if intent == Intent::BusinessAnalysis {
        let analysis = Thinker::analyze_business(message).await?;
        return Ok(analysis);
    }
    
    // 4. For recommendations, use AI
    if intent == Intent::Recommendation {
        let recommendation = Thinker::get_ai_recommendation(
            message,
            Some("seafood, spicy")
        ).await?;
        return Ok(recommendation);
    }
    
    // 5. Fallback to simple response
    Ok(format!("Mood: {}, Intent: {:?}", mood, intent))
}
```

---

## 🧪 Testing

```bash
# 1. Set API key
export GROQ_API_KEY="your_key"

# 2. Run tests
cargo test --lib ai::core::groq -- --ignored

# 3. Test integration
cargo test --lib ai::thinker -- --nocapture
```

---

## ⚡ Performance

**Response Times (Groq devnet):**
- Llama 8B: ~200-500ms
- Llama 70B: ~1-2s
- Mixtral: ~800ms-1.5s

**Rate Limits:**
- Free tier: 30 requests/minute
- Paid tier: Higher limits

---

## 🎯 Use Cases in FodiFood

### 1. **Customer Chat** → `Thinker::think()`
```rust
"What should I order for dinner?" → AI recommends based on context
```

### 2. **Business Analytics** → `Thinker::analyze_business()`
```rust
Sales data → AI generates insights and recommendations
```

### 3. **Menu Recommendations** → `Thinker::get_ai_recommendation()`
```rust
User preferences → Personalized dish suggestions
```

### 4. **Entity Extraction** → `Thinker::extract_with_ai()`
```rust
"I want a spicy seafood dish" → Extracts: ["spicy", "seafood"]
```

---

## 🔒 Error Handling

All Groq functions have fallback logic:

```rust
match Thinker::think(prompt).await {
    Ok(response) => response,
    Err(e) => {
        tracing::error!("AI failed: {}", e);
        "AI temporarily unavailable".to_string()  // Graceful fallback
    }
}
```

---

## 📝 Next Steps

1. ✅ Groq integration complete
2. ⏳ Add streaming support (SSE)
3. ⏳ Implement conversation memory
4. ⏳ Add multi-turn context
5. ⏳ Fine-tune prompts for food domain

---

**Status**: ✅ Ready to use!  
**Date**: 20 октября 2025  
**Model**: Llama 3.1 70B + 8B Instant
