-- Make password_hash nullable for OAuth users
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- Add GitHub OAuth fields
ALTER TABLE users ADD COLUMN github_id BIGINT UNIQUE;
ALTER TABLE users ADD COLUMN avatar_url TEXT;
