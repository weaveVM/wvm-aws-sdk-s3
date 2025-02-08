-- ACCOUNTS
CREATE TABLE accounts (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    account_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX idx_accounts_status ON accounts(is_active);

-- ACCESS KEYS
CREATE TABLE access_keys (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    owner_id BIGINT UNSIGNED NOT NULL,
    access_key VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX idx_access_keys_owner ON access_keys(owner_id);
CREATE INDEX idx_access_keys_lookup ON access_keys(access_key);
CREATE INDEX idx_access_keys_status ON access_keys(is_active);

-- BUCKET INDEX
CREATE TABLE bucket_index (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    account_id BIGINT UNSIGNED NOT NULL,
    bucket_name VARCHAR(255) NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    block_number BIGINT UNSIGNED,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

CREATE INDEX idx_bucket_account ON bucket_index(account_id);
CREATE INDEX idx_bucket_tx_block ON bucket_index(tx_hash, block_number);

-- OBJECT INDEX
CREATE TABLE object_index (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    bucket_id BIGINT UNSIGNED NOT NULL,
    object_key VARCHAR(1024) NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    block_number BIGINT UNSIGNED,
    size_bytes BIGINT UNSIGNED,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    metadata JSON
);

CREATE INDEX idx_object_bucket ON object_index(bucket_id);
CREATE INDEX idx_object_tx_block ON object_index(tx_hash, block_number);
CREATE INDEX idx_object_status ON object_index(is_deleted);
