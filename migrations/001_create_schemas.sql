-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create AI Schema for Rust AI modules
CREATE SCHEMA IF NOT EXISTS ai;

-- Create Blockchain Schema for Solana/FODI operations  
CREATE SCHEMA IF NOT EXISTS blockchain;

-- Create Analytics Schema for metrics and events
CREATE SCHEMA IF NOT EXISTS analytics;

-- Grant permissions to database owner
GRANT USAGE ON SCHEMA ai TO neondb_owner;
GRANT USAGE ON SCHEMA blockchain TO neondb_owner;
GRANT USAGE ON SCHEMA analytics TO neondb_owner;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA ai TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA blockchain TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA analytics TO neondb_owner;

GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA ai TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA blockchain TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA analytics TO neondb_owner;

-- Set search path to include new schemas
ALTER DATABASE neondb SET search_path TO public, ai, blockchain, analytics;

COMMENT ON SCHEMA ai IS 'AI cache, memory, conversations, and learning data';
COMMENT ON SCHEMA blockchain IS 'FODI token ledger, wallets, NFTs, and rewards';
COMMENT ON SCHEMA analytics IS 'Metrics, events, and aggregations';
