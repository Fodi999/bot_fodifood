# 🧠 AI Module Structure

**Path**: `/src/ai/`  
**Purpose**: Intelligent conversation handling and business analysis

## 📁 Directory Structure

```
src/ai/
├── Core Files (7)
│   ├── mod.rs                    # Main AI engine, exports all modules
│   ├── intent_handler.rs         # Intent routing and handler registration
│   ├── intents.rs                # Intent type definitions
│   ├── memory.rs                 # In-memory conversation context
│   ├── persistent_memory.rs      # Database-backed conversation storage
│   ├── analysis.rs               # Business intelligence and analytics
│   ├── admin_assistant.rs        # Admin control commands
│   └── thinker.rs                # AI reasoning and decision making
│
├── modules/ (9 files)            # Intent implementation modules
│   ├── mod.rs                    # Module exports
│   ├── menu.rs                   # Menu search and display
│   ├── orders.rs                 # Order creation and tracking
│   ├── smalltalk.rs              # Casual conversation
│   ├── business.rs               # Business management
│   ├── analytics.rs              # Statistics and reports
│   ├── recommendations.rs        # AI recommendations
│   ├── news.rs                   # News and updates
│   ├── orders.rs.bak             # Backup file
│   └── smalltalk.rs.bak          # Backup file
│
└── rules/ (7 files)              # Business rules and validation
    ├── mod.rs                    # Rules exports
    ├── common.rs                 # Shared validation rules
    ├── menu.rs                   # Menu-related rules
    ├── orders.rs                 # Order validation rules
    ├── analytics.rs              # Analytics rules
    ├── recommendations.rs        # Recommendation rules
    └── smalltalk.rs              # Conversation rules
```

## 🎯 Core Components

### 1. **Intent Handler** (`intent_handler.rs`)
```rust
// Registers and routes user intents
pub struct IntentHandler {
    handlers: HashMap<String, Box<dyn Fn(...)>>,
}

// 17 registered intents:
- help
- showmenu
- searchmenu
- searchbyingredient
- createorder
- orderstatus
- cancelorder
- checkingredients
- stockstatus
- getstatistics
- salesanalysis
- analyzebusiness
- comparebusinesses
- businessinsights
- recommendations
- deliveryinfo
- smalltalk
```

### 2. **Intent Types** (`intents.rs`)
```rust
pub enum Intent {
    ShowMenu,
    SearchMenu(String),
    CreateOrder { items: Vec<OrderItem> },
    OrderStatus(String),
    Help,
    SmallTalk(String),
    GetStatistics { period: String },
    // ... and more
}
```

### 3. **Memory System**

#### In-Memory (`memory.rs`)
- Conversation context during session
- Quick access to recent messages
- User preferences and state

#### Persistent (`persistent_memory.rs`)
- Database storage (sled)
- Long-term conversation history
- User profile and patterns

### 4. **Analysis Engine** (`analysis.rs`)
```rust
// Business intelligence functions:
- analyze_sales_trends()
- calculate_roi()
- predict_demand()
- generate_insights()
- compare_businesses()
```

### 5. **Thinker** (`thinker.rs`)
```rust
// AI decision making:
- reason_about_intent()
- generate_response()
- apply_context()
- learn_from_feedback()
```

## 📦 Modules (Intent Implementations)

### 1. **Menu Module** (`modules/menu.rs`)
```rust
// Handles menu-related intents:
pub async fn show_menu(business_id: &str) -> Result<MenuResponse>
pub async fn search_menu(query: &str) -> Result<Vec<MenuItem>>
pub async fn search_by_ingredient(ingredient: &str) -> Result<Vec<MenuItem>>
```

**Use Cases:**
- "Show me the menu"
- "Find pizza"
- "What has tomatoes?"

### 2. **Orders Module** (`modules/orders.rs`)
```rust
// Order management:
pub async fn create_order(user_id: &str, items: Vec<OrderItem>) -> Result<Order>
pub async fn get_order_status(order_id: &str) -> Result<OrderStatus>
pub async fn cancel_order(order_id: &str) -> Result<bool>
```

**Use Cases:**
- "I want to order pizza"
- "Where's my order #123?"
- "Cancel my order"

### 3. **SmallTalk Module** (`modules/smalltalk.rs`)
```rust
// Casual conversation:
pub async fn handle_greeting(msg: &str) -> String
pub async fn handle_thanks(msg: &str) -> String
pub async fn handle_general_chat(msg: &str) -> String
```

**Use Cases:**
- "Hello!"
- "Thank you"
- "How are you?"

### 4. **Business Module** (`modules/business.rs`)
```rust
// Business operations:
pub async fn register_business(data: BusinessData) -> Result<Business>
pub async fn update_business(id: &str, data: BusinessData) -> Result<Business>
pub async fn get_business_info(id: &str) -> Result<Business>
```

**Use Cases:**
- "Register my restaurant"
- "Update business hours"
- "Show business info"

### 5. **Analytics Module** (`modules/analytics.rs`)
```rust
// Statistics and reports:
pub async fn get_statistics(period: &str) -> Result<Statistics>
pub async fn analyze_sales(business_id: &str) -> Result<SalesAnalysis>
pub async fn compare_businesses(ids: Vec<String>) -> Result<Comparison>
```

**Use Cases:**
- "Show me this month's stats"
- "Analyze my sales"
- "Compare with competitors"

### 6. **Recommendations Module** (`modules/recommendations.rs`)
```rust
// AI-powered suggestions:
pub async fn recommend_dishes(user_id: &str) -> Result<Vec<MenuItem>>
pub async fn recommend_optimizations(business_id: &str) -> Result<Vec<Suggestion>>
pub async fn recommend_pricing(item: &MenuItem) -> Result<PriceRecommendation>
```

**Use Cases:**
- "What should I order?"
- "How can I improve my business?"
- "Optimal price for this dish?"

### 7. **News Module** (`modules/news.rs`)
```rust
// Updates and notifications:
pub async fn get_news(category: &str) -> Result<Vec<NewsItem>>
pub async fn get_business_updates(business_id: &str) -> Result<Vec<Update>>
```

**Use Cases:**
- "What's new?"
- "Business updates"

## 🛡️ Rules (Validation & Business Logic)

### 1. **Common Rules** (`rules/common.rs`)
```rust
// Shared validation:
pub fn validate_user_input(input: &str) -> Result<()>
pub fn sanitize_text(text: &str) -> String
pub fn check_permissions(user: &User, action: &str) -> bool
```

### 2. **Menu Rules** (`rules/menu.rs`)
```rust
// Menu validation:
pub fn validate_menu_item(item: &MenuItem) -> Result<()>
pub fn check_availability(item: &MenuItem) -> bool
pub fn validate_price(price: f64) -> Result<()>
```

### 3. **Order Rules** (`rules/orders.rs`)
```rust
// Order validation:
pub fn validate_order(order: &Order) -> Result<()>
pub fn check_minimum_order(total: f64) -> bool
pub fn validate_delivery_address(address: &str) -> Result<()>
```

### 4. **Analytics Rules** (`rules/analytics.rs`)
```rust
// Data validation:
pub fn validate_date_range(start: &str, end: &str) -> Result<()>
pub fn validate_metrics(metrics: &[String]) -> Result<()>
```

## 🔄 Data Flow

```
User Message
    ↓
Intent Handler (detect intent)
    ↓
Route to appropriate Module
    ↓
Apply Rules (validation)
    ↓
Execute Business Logic
    ↓
Update Memory (context)
    ↓
Generate Response
    ↓
Return to User
```

## 📊 Intent Statistics

Currently registered: **17 intents**

| Category | Count | Modules |
|----------|-------|---------|
| **Menu** | 3 | showmenu, searchmenu, searchbyingredient |
| **Orders** | 3 | createorder, orderstatus, cancelorder |
| **Analytics** | 5 | getstatistics, salesanalysis, analyzebusiness, comparebusinesses, businessinsights |
| **Inventory** | 2 | checkingredients, stockstatus |
| **General** | 3 | help, deliveryinfo, smalltalk |
| **AI** | 1 | recommendations |

## 🧪 Testing Examples

### Menu Intent:
```bash
User: "Show me the menu"
→ Intent: ShowMenu
→ Module: modules/menu.rs::show_menu()
→ Response: [List of menu items]
```

### Order Intent:
```bash
User: "I want 2 pizzas and a coke"
→ Intent: CreateOrder { items: [...] }
→ Module: modules/orders.rs::create_order()
→ Rules: validate_order(), check_minimum_order()
→ Response: "Order #123 created, total: $25.50"
```

### Analytics Intent:
```bash
User: "Analyze my sales this month"
→ Intent: SalesAnalysis { period: "month" }
→ Module: modules/analytics.rs::analyze_sales()
→ Analysis: analysis.rs::analyze_sales_trends()
→ Response: [Detailed sales report]
```

## 🔮 AI Capabilities

### 1. **Natural Language Understanding**
- Intent detection from free text
- Entity extraction (items, dates, numbers)
- Context awareness (remembers conversation)

### 2. **Business Intelligence**
- Sales trend analysis
- Demand prediction
- ROI calculation
- Competitive analysis

### 3. **Personalization**
- User preference learning
- Personalized recommendations
- Adaptive responses

### 4. **Multi-language Support** (planned)
- Russian
- English
- More languages...

## 📝 Adding New Intent

```rust
// 1. Define intent in intents.rs
pub enum Intent {
    NewIntent { param: String },
}

// 2. Create module in modules/
// modules/new_feature.rs
pub async fn handle_new_intent(param: &str) -> Result<Response> {
    // Implementation
}

// 3. Add rules in rules/
// rules/new_feature.rs
pub fn validate_param(param: &str) -> Result<()> {
    // Validation
}

// 4. Register in intent_handler.rs
intent_handler.register("newintent", |msg| {
    modules::new_feature::handle_new_intent(&msg.param)
});
```

## 🎯 Next Steps

### High Priority:
- [ ] Add NFT-based access control
- [ ] Integrate with Bank module for payments
- [ ] Multi-language support

### Medium Priority:
- [ ] Machine learning for better recommendations
- [ ] Voice command support
- [ ] Image recognition for menu items

### Low Priority:
- [ ] Sentiment analysis
- [ ] Automated testing suite
- [ ] Performance monitoring

---

**Total Files**: 25 files (7 core + 9 modules + 7 rules + 2 backups)  
**Total Intents**: 17 registered handlers  
**Lines of Code**: ~2000+ lines  
**Status**: ✅ Fully operational
