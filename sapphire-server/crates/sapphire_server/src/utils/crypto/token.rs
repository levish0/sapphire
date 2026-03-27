use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt;

pub fn generate_secure_token() -> String {
    let token_bytes: [u8; 32] = rand::rng().random();
    URL_SAFE_NO_PAD.encode(token_bytes)
}

pub fn generate_secure_token_with_length(byte_length: usize) -> String {
    let token_bytes: Vec<u8> = (0..byte_length).map(|_| rand::rng().random()).collect();
    URL_SAFE_NO_PAD.encode(&token_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secure_token() {
        let token = generate_secure_token();
        // 32 bytes -> 43 chars in base64 (no padding)
        assert_eq!(token.len(), 43);

        let token2 = generate_secure_token();
        assert_ne!(token, token2);
    }

    #[test]
    fn test_generate_secure_token_with_length() {
        let token = generate_secure_token_with_length(16);
        // 16 bytes -> 22 chars in base64 (no padding)
        assert_eq!(token.len(), 22);
    }
}
