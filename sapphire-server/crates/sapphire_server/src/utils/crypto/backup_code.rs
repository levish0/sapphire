use sapphire_config::ServerConfig;

pub fn hash_backup_code(code: &str) -> String {
    let config = ServerConfig::get();
    let key = blake3::derive_key(
        "sevenwiki totp backup code v1",
        config.totp_secret.as_bytes(),
    );
    let mut hasher = blake3::Hasher::new_keyed(&key);
    hasher.update(code.as_bytes());
    hasher.finalize().to_hex().to_string()
}

pub fn hash_backup_codes(codes: &[String]) -> Vec<String> {
    codes.iter().map(|c| hash_backup_code(c)).collect()
}

pub fn verify_backup_code(code: &str, stored_hashes: &[String]) -> Option<usize> {
    let input_hash = hash_backup_code(code);
    stored_hashes.iter().position(|h| h == &input_hash)
}
