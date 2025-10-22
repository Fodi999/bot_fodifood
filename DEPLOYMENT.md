# 🚀 Deployment Guide - Shuttle.rs

## 📋 Prerequisites

1. **Shuttle Account**: https://shuttle.rs
2. **Shuttle CLI**: 
   ```bash
   cargo install cargo-shuttle
   ```
3. **Login**:
   ```bash
   shuttle login
   ```

---

## ⚡ Quick Deploy (One Command)

```bash
shuttle deploy
```

That's it! 🎉

---

## 🔧 Step-by-Step Deployment

### 1. Configure Secrets

Shuttle использует **Secrets.toml** для environment variables.

Создай/обнови `Secrets.toml`:

```toml
# Database
DATABASE_URL = "postgresql://neondb_owner:npg_dz4Gl8ZhPLbX@ep-soft-mud-agon8wu3-pooler.c-2.eu-central-1.aws.neon.tech/neondb?sslmode=require"

# AI Configuration
GROQ_API_KEY = "your_groq_api_key_here"
GROQ_MODEL = "llama-3.1-8b-instant"

# Solana
SOLANA_NETWORK = "devnet"
SOLANA_RPC_URL = "https://api.devnet.solana.com"
KEYPAIR_PATH = "./data/keypair.json"

# Backend (если отдельно)
GO_BACKEND_URL = "https://yeasty-madelaine-fodi999-671ccdf5.koyeb.app"

# Orchestrator (если нужен)
ORCHESTRATOR_ENABLED = "false"
```

⚠️ **Important**: `Secrets.toml` в `.gitignore` - не коммитим!

---

### 2. Deploy to Shuttle

```bash
# Deploy в production
shuttle deploy

# Или с конкретным именем
shuttle deploy --name fodifood-bot
```

**Что происходит:**
1. ✅ Код компилируется в Shuttle cloud
2. ✅ Secrets загружаются из Secrets.toml
3. ✅ Создаётся production URL
4. ✅ Автоматический SSL certificate
5. ✅ Health checks настраиваются

**Output:**
```
   Deploying fodifood-bot to Shuttle...
   Compiling...
   Building release binary...
   Uploading...
   
   ✅ Deployment successful!
   
   🌐 URL: https://fodifood-bot.shuttleapp.rs
   📊 Logs: shuttle logs
```

---

### 3. Verify Deployment

```bash
# Check logs
shuttle logs

# Check status
shuttle status

# Test health endpoint
curl https://fodifood-bot.shuttleapp.rs/health
```

---

## 📊 Deployment Commands

### Basic Commands

```bash
# Deploy
shuttle deploy

# View logs (live)
shuttle logs

# View logs (follow)
shuttle logs --follow

# Check status
shuttle status

# Stop service
shuttle stop

# Restart service
shuttle restart

# Delete deployment
shuttle delete
```

### Project Management

```bash
# List projects
shuttle project list

# Create new project
shuttle project create

# Delete project
shuttle project delete

# Project status
shuttle project status
```

### Secrets Management

```bash
# View secrets (не показывает значения)
shuttle secrets list

# Add secret
shuttle secrets add KEY=value

# Remove secret
shuttle secrets remove KEY
```

---

## 🔄 CI/CD Pipeline

### GitHub Actions Workflow

Создай `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Shuttle

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install Shuttle CLI
        run: cargo install cargo-shuttle
      
      - name: Login to Shuttle
        env:
          SHUTTLE_API_KEY: ${{ secrets.SHUTTLE_API_KEY }}
        run: |
          shuttle login --api-key $SHUTTLE_API_KEY
      
      - name: Create Secrets.toml
        env:
          DATABASE_URL: ${{ secrets.DATABASE_URL }}
          GROQ_API_KEY: ${{ secrets.GROQ_API_KEY }}
          GO_BACKEND_URL: ${{ secrets.GO_BACKEND_URL }}
        run: |
          cat > Secrets.toml << EOF
          DATABASE_URL = "$DATABASE_URL"
          GROQ_API_KEY = "$GROQ_API_KEY"
          GROQ_MODEL = "llama-3.1-8b-instant"
          SOLANA_NETWORK = "devnet"
          SOLANA_RPC_URL = "https://api.devnet.solana.com"
          GO_BACKEND_URL = "$GO_BACKEND_URL"
          ORCHESTRATOR_ENABLED = "false"
          EOF
      
      - name: Deploy to Shuttle
        run: shuttle deploy --allow-dirty
      
      - name: Health Check
        run: |
          sleep 10
          curl -f https://fodifood-bot.shuttleapp.rs/health || exit 1
      
      - name: Notify Success
        if: success()
        run: echo "✅ Deployment successful!"
```

**Setup GitHub Secrets:**
1. Go to repo → Settings → Secrets
2. Add:
   - `SHUTTLE_API_KEY` (from `shuttle login`)
   - `DATABASE_URL`
   - `GROQ_API_KEY`
   - `GO_BACKEND_URL`

---

## 🌍 Environment-Specific Deploys

### Development

```bash
# Local development
cargo run --bin local

# Or with environment
ENV=development cargo run --bin local
```

### Staging

```bash
# Deploy to staging (если есть отдельный проект)
shuttle deploy --name fodifood-bot-staging
```

### Production

```bash
# Deploy to production
shuttle deploy --name fodifood-bot
```

---

## 📦 Database Migrations on Deploy

### Option 1: Run Migrations Automatically

Добавь в `src/main.rs`:

```rust
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleAxum {
    // Load secrets
    let database_url = secrets.get("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Run migrations
    info!("Running database migrations...");
    run_migrations(&database_url).await?;
    
    // Initialize app
    let state = AppState::new(config);
    let router = create_router(state);
    
    Ok(router.into())
}

async fn run_migrations(database_url: &str) -> Result<()> {
    let pool = PgPool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}
```

### Option 2: Run Migrations Manually

```bash
# Before deploy
./migrate.sh

# Then deploy
shuttle deploy
```

---

## 🔍 Monitoring

### Shuttle Dashboard

Visit: https://console.shuttle.rs

**Features:**
- 📊 Resource usage (CPU, Memory)
- 📝 Real-time logs
- 🔄 Deployment history
- ⚙️ Configuration
- 🚦 Health status

### Custom Monitoring

Add to `src/main.rs`:

```rust
// Prometheus metrics endpoint
router.route("/metrics", get(metrics_handler))
```

Then setup Grafana Cloud:
```bash
# Scrape metrics from Shuttle
https://fodifood-bot.shuttleapp.rs/metrics
```

---

## 🛠️ Troubleshooting

### Build Fails

**Error:**
```
error: could not compile `fodifood-bot`
```

**Solution:**
```bash
# Test build locally first
cargo build --release

# Check for compile errors
cargo check
```

---

### Secrets Not Found

**Error:**
```
thread 'main' panicked at 'DATABASE_URL must be set'
```

**Solution:**
```bash
# Verify Secrets.toml exists
ls -la Secrets.toml

# Re-deploy with secrets
shuttle deploy
```

---

### Service Won't Start

**Error:**
```
Service failed health check
```

**Solution:**
```bash
# Check logs
shuttle logs --follow

# Common issues:
# - Database connection failed
# - Missing secrets
# - Port binding error

# Verify health endpoint locally
cargo run --bin local
curl http://127.0.0.1:8000/health
```

---

### Database Connection Issues

**Error:**
```
Error: Failed to connect to PostgreSQL
```

**Solution:**
```bash
# Test connection string
psql "postgresql://..."

# Check firewall rules (Neon)
# - Add Shuttle IP to allowlist

# Verify DATABASE_URL in Secrets.toml
```

---

## 🎯 Deployment Checklist

### Pre-Deploy
- [ ] All tests passing (`cargo test`)
- [ ] Local build successful (`cargo build --release`)
- [ ] Secrets.toml configured
- [ ] Database migrations ready
- [ ] Health endpoint working

### Deploy
- [ ] Run `shuttle deploy`
- [ ] Wait for build completion
- [ ] Verify deployment URL

### Post-Deploy
- [ ] Health check passes
- [ ] Test API endpoints
- [ ] Check logs for errors
- [ ] Verify database connection
- [ ] Test multi-agent system
- [ ] Monitor metrics

---

## 📊 Shuttle.toml Configuration

```toml
name = "bot-fodifood"
type = "web-axum"

[build]
# Add build assets if needed
assets = ["data/"]

[deploy]
shuttle-name = "fodifood-bot"

# Optional: Specify Rust version
# rust-version = "1.75"

# Optional: Custom build command
# build-command = "cargo build --release --bin fodifood-bot"
```

---

## 🚀 Production URLs

After deployment:

| Service | URL |
|---------|-----|
| Rust Bot | https://fodifood-bot.shuttleapp.rs |
| Health Check | https://fodifood-bot.shuttleapp.rs/health |
| API v1 | https://fodifood-bot.shuttleapp.rs/api/v1/* |
| Metrics | https://fodifood-bot.shuttleapp.rs/metrics |
| WebSocket | wss://fodifood-bot.shuttleapp.rs/ws |
| Admin API | https://fodifood-bot.shuttleapp.rs/api/v1/admin/* |

---

## 💡 Pro Tips

1. **Use Environment Variables**
   ```rust
   let env = std::env::var("ENV").unwrap_or("production".into());
   ```

2. **Database Connection Pool**
   ```rust
   PgPoolOptions::new()
       .max_connections(5) // Lower on Shuttle
       .connect(database_url).await?
   ```

3. **Graceful Shutdown**
   ```rust
   // Shuttle handles this automatically
   ```

4. **Resource Limits**
   - Memory: 512MB (default)
   - CPU: Shared
   - Consider upgrading for production load

5. **Logging**
   ```rust
   tracing::info!("Deployment info");
   tracing::error!("Critical errors");
   ```

---

## 📞 Support

- **Shuttle Docs**: https://docs.shuttle.rs
- **Shuttle Discord**: https://discord.gg/shuttle
- **GitHub Issues**: https://github.com/shuttle-hq/shuttle/issues

---

## 🎉 Summary

```bash
# Complete deployment in 3 steps:

# 1. Configure secrets
nano Secrets.toml

# 2. Deploy
shuttle deploy

# 3. Verify
curl https://fodifood-bot.shuttleapp.rs/health

# Done! 🚀
```

**Deployment time**: ~2-5 minutes  
**Zero downtime**: ✅  
**Auto SSL**: ✅  
**Built-in monitoring**: ✅  
**One-click rollback**: ✅

---

**Ready for production! 🎯**
