-- Add up migration script here
CREATE TABLE nfts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    image_url VARCHAR(512) NOT NULL,
    metadata_url VARCHAR(512),
    mint_address VARCHAR(64),
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_nfts_user_id ON nfts(user_id);
CREATE INDEX idx_nfts_status ON nfts(status);
CREATE INDEX idx_nfts_created_at ON nfts(created_at);
