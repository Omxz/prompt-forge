-- Add usage tracking to agents table
-- Version: 002_add_usage_tracking

-- Add usage tracking columns to agents table (only if they don't exist)
-- SQLite doesn't have ALTER TABLE IF NOT EXISTS, so we need to handle errors gracefully
-- This will fail silently if columns already exist

-- Try to add columns (will fail if they exist, but that's ok)
-- We'll check in the Rust code if migration is needed

ALTER TABLE agents ADD COLUMN usage_count INTEGER DEFAULT 0;
ALTER TABLE agents ADD COLUMN last_used_at TEXT DEFAULT NULL;

-- Create index for sorting by usage
CREATE INDEX IF NOT EXISTS idx_agents_usage_count ON agents(usage_count DESC);
CREATE INDEX IF NOT EXISTS idx_agents_last_used ON agents(last_used_at DESC);
