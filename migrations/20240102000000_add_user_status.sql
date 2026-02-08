-- Add status column with default 'Active'
ALTER TABLE users 
ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'Active'
CHECK (status IN ('Active', 'Suspended'));
