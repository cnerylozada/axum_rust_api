-- Add migration script here
TRUNCATE TABLE users;

-- Drop old columns
ALTER TABLE users
    DROP COLUMN IF EXISTS username,
    DROP COLUMN IF EXISTS age,
    DROP COLUMN IF EXISTS wallet,
    DROP COLUMN IF EXISTS email,
    DROP COLUMN IF EXISTS role;

-- Add new columns
ALTER TABLE users
    ADD COLUMN wallet VARCHAR(255),
    ADD COLUMN email VARCHAR(255),
    ADD COLUMN role VARCHAR(10) NOT NULL DEFAULT 'USER'
        CHECK (role IN ('USER', 'ADMIN'));