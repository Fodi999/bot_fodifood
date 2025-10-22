-- Cache cleanup function (removes expired entries)
CREATE OR REPLACE FUNCTION ai.cleanup_expired_cache()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM ai.cache_entries
    WHERE expires_at < NOW();
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION ai.cleanup_expired_cache IS 'Delete expired cache entries and return count';

-- Optional: Create a scheduled job using pg_cron (if extension is available)
-- Uncomment if pg_cron is installed:
-- SELECT cron.schedule('cleanup-ai-cache', '0 * * * *', 'SELECT ai.cleanup_expired_cache()');

-- Trigger to auto-update updated_at
CREATE OR REPLACE FUNCTION ai.update_modified_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_memory_facts_timestamp
    BEFORE UPDATE ON ai.memory_facts
    FOR EACH ROW
    EXECUTE FUNCTION ai.update_modified_timestamp();

COMMENT ON FUNCTION ai.update_modified_timestamp IS 'Auto-update updated_at timestamp on row modification';
