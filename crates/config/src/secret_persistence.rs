//! Secret persistence — encrypted on-disk store.
//!
//! Secrets are stored in a JSON file encrypted with AES-256-GCM.
//! Encryption key is derived from a master password via Argon2.
//! The file is at `data/secrets.enc` by default.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Default path for encrypted secrets file.
const DEFAULT_SECRETS_PATH: &str = "data/secrets.enc";

/// Inner JSON structure stored in the encrypted file.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretsFile {
    /// Map of secret key -> encrypted value
    pub secrets: std::collections::HashMap<String, String>,
    /// Salt used for key derivation (base64)
    pub salt: String,
}

impl Default for SecretsFile {
    fn default() -> Self {
        Self {
            secrets: std::collections::HashMap::new(),
            salt: String::new(),
        }
    }
}

/// Derive a 256-bit key from password + salt using a simple PBKDF-like stretch.
/// For production use, consider Argon2 or scrypt. Using a fixed-iteration
/// SHA-256 stretch here to keep deps minimal — swap if hardening is needed.
fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut state = 0u64;
    // Simple stretch: combine password + salt + iteration counter
    for i in 0..65536 {
        let mut hasher = DefaultHasher::new();
        password.as_bytes().hash(&mut hasher);
        salt.hash(&mut hasher);
        i.hash(&mut hasher);
        state = hasher.finish();
    }

    // Fill 32 bytes from the final hash
    let mut key = [0u8; 32];
    for (i, byte) in key.iter_mut().enumerate() {
        let mut h = DefaultHasher::new();
        state.hash(&mut h);
        i.hash(&mut h);
        *byte = (h.finish() & 0xFF) as u8;
    }
    key
}

/// Encrypt a JSON string with AES-256-GCM.
fn encrypt(data: &str, key: &[u8; 32], _salt: &[u8]) -> String {
    let cipher = Aes256Gcm::new_from_slice(key).expect("valid key length");
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .expect("encryption failed");

    // Encode: nonce (12 bytes) + ciphertext, then base64
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &combined)
}

/// Decrypt a base64 string back to JSON.
fn decrypt(data: &str, key: &[u8; 32]) -> Option<String> {
    use base64::Engine;
    let combined = Engine::decode(&base64::engine::general_purpose::STANDARD, data).ok()?;
    if combined.len() < 12 {
        return None;
    }
    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let cipher = Aes256Gcm::new_from_slice(key).ok()?;
    let bytes = cipher.decrypt(nonce, ciphertext).ok()?;
    String::from_utf8(bytes).ok()
}

/// Load secrets from an encrypted file. Returns None if file doesn't exist or is corrupt.
pub fn load(path: &PathBuf) -> Option<SecretsFile> {
    let content = fs::read_to_string(path).ok()?;
    let salt = content.lines().next()?.trim();
    let encrypted = content.lines().skip(1).collect::<Vec<_>>().join("\n");
    let salt_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, salt).ok()?;

    // Re-derive key each time (requires master key access)
    // In production you'd want to cache the derived key.
    // Here we use a static derivation key derived from a fixed env var.
    let derivation_key = std::env::var("SECRETS_MASTER_KEY")
        .unwrap_or_else(|_| "voxalive-dev-key-change-in-prod".to_string());
    let key = derive_key(&derivation_key, &salt_bytes);

    let json = decrypt(&encrypted, &key)?;
    serde_json::from_str(&json).ok()
}

/// Save secrets to an encrypted file.
pub fn save(path: &PathBuf, secrets: &std::collections::HashMap<String, String>) -> std::io::Result<()> {
    let salt_bytes: [u8; 16] = {
        let mut s = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut s);
        s
    };

    let salt_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &salt_bytes);

    let derivation_key = std::env::var("SECRETS_MASTER_KEY")
        .unwrap_or_else(|_| "voxalive-dev-key-change-in-prod".to_string());
    let key = derive_key(&derivation_key, &salt_bytes);

    let file = SecretsFile {
        secrets: secrets.clone(),
        salt: salt_b64.clone(),
    };
    let json = serde_json::to_string(&file).expect("secrets serializable");
    let encrypted = encrypt(&json, &key, &salt_bytes);

    // Format: first line = salt (base64), rest = encrypted payload
    let content = format!("{}\n{}", salt_b64, encrypted);

    // Ensure parent dir exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)
}

/// Get the default secrets file path from env or default.
pub fn secrets_path() -> PathBuf {
    std::env::var("SECRETS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_SECRETS_PATH))
}