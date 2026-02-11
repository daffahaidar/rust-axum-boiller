-- Add Google OAuth field
ALTER TABLE users ADD COLUMN google_id VARCHAR(255) UNIQUE;
