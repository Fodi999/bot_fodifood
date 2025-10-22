-- Role-based access control for Go and Rust applications

-- Note: This is optional - only run if you want separate roles
-- By default, neondb_owner has full access to everything

-- Uncomment if you want to create separate app roles:

/*
-- Create application roles
CREATE ROLE app_go LOGIN PASSWORD 'change_me_go_password';
CREATE ROLE app_rust LOGIN PASSWORD 'change_me_rust_password';

-- Grant schema usage
GRANT USAGE ON SCHEMA public, ai, blockchain, analytics TO app_go, app_rust;

-- Go backend: Full access to public schema
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO app_go;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO app_go;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_go;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO app_go;

-- Rust: Read-only to public (can query users, businesses, orders)
GRANT SELECT ON ALL TABLES IN SCHEMA public TO app_rust;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO app_rust;

-- Rust: Full access to ai, blockchain, analytics schemas
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA ai TO app_rust;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA blockchain TO app_rust;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA analytics TO app_rust;

GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA ai TO app_rust;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA blockchain TO app_rust;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA analytics TO app_rust;

ALTER DEFAULT PRIVILEGES IN SCHEMA ai GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_rust;
ALTER DEFAULT PRIVILEGES IN SCHEMA blockchain GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_rust;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_rust;

ALTER DEFAULT PRIVILEGES IN SCHEMA ai GRANT USAGE, SELECT ON SEQUENCES TO app_rust;
ALTER DEFAULT PRIVILEGES IN SCHEMA blockchain GRANT USAGE, SELECT ON SEQUENCES TO app_rust;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT USAGE, SELECT ON SEQUENCES TO app_rust;

-- Grant execute on functions
GRANT EXECUTE ON FUNCTION ai.cleanup_expired_cache() TO app_rust;
GRANT EXECUTE ON FUNCTION analytics.refresh_daily_stats() TO app_rust;

COMMENT ON ROLE app_go IS 'Go backend application - full access to public schema';
COMMENT ON ROLE app_rust IS 'Rust application - read public, write ai/blockchain/analytics';
*/

-- For now, just ensure neondb_owner has everything
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA ai TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA blockchain TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA analytics TO neondb_owner;

GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA ai TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA blockchain TO neondb_owner;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA analytics TO neondb_owner;

GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA ai TO neondb_owner;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA analytics TO neondb_owner;
