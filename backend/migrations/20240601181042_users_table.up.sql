-- Add up migration script here
-- Trigger function to update the timestamp on the 'questions' table
CREATE OR REPLACE FUNCTION update_user_timestamp() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Table to store the users
CREATE TABLE IF NOT EXISTS users(
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    is_staff BOOLEAN DEFAULT FALSE,
    is_superuser BOOLEAN DEFAULT FALSE,
    thumbnail TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS users_id_is_active_indx ON users (id, is_active);
CREATE INDEX IF NOT EXISTS users_email_is_active_indx ON users (email, is_active);

-- Trigger to update the timestamp on the 'users' table
CREATE TRIGGER update_user_timestamp BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_user_timestamp();