#!/bin/bash

# 🗄️ Database Migration Script
# Applies all SQL migrations to Neon PostgreSQL

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "🗄️  FodiFood Database Migration"
echo "================================"
echo ""

# Load DATABASE_URL from .env
if [ -f .env ]; then
    export $(cat .env | grep DATABASE_URL | xargs)
    echo -e "${GREEN}✅ Loaded DATABASE_URL from .env${NC}"
else
    echo -e "${RED}❌ .env file not found${NC}"
    exit 1
fi

if [ -z "$DATABASE_URL" ]; then
    echo -e "${RED}❌ DATABASE_URL not set in .env${NC}"
    exit 1
fi

echo -e "${YELLOW}📡 Target database: ${DATABASE_URL:0:50}...${NC}"
echo ""

# Migration files in order
MIGRATIONS=(
    "001_create_schemas.sql"
    "002_create_ai_tables.sql"
    "003_create_blockchain_tables.sql"
    "004_create_analytics_tables.sql"
    "005_create_functions.sql"
    "006_permissions.sql"
)

echo "📋 Found ${#MIGRATIONS[@]} migration files"
echo ""

# Apply each migration
for migration in "${MIGRATIONS[@]}"; do
    migration_file="migrations/$migration"
    
    if [ ! -f "$migration_file" ]; then
        echo -e "${RED}❌ Migration file not found: $migration_file${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}🔄 Applying: $migration${NC}"
    
    if psql "$DATABASE_URL" -f "$migration_file" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Success: $migration${NC}"
    else
        echo -e "${RED}❌ Failed: $migration${NC}"
        echo -e "${RED}   Check the error above for details${NC}"
        exit 1
    fi
    
    echo ""
done

echo ""
echo -e "${GREEN}🎉 All migrations applied successfully!${NC}"
echo ""
echo "📊 Database Schema:"
echo "  ├── public (Go backend - users, businesses, orders)"
echo "  ├── ai (Rust AI - cache, memory, conversations)"
echo "  ├── blockchain (Rust Crypto - FODI ledger, wallets, NFTs)"
echo "  └── analytics (Rust Analytics - metrics, events)"
echo ""
echo -e "${GREEN}✅ Database is ready!${NC}"
