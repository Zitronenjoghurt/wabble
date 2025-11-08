use crate::crypto::secret::Secret;
use crate::message::server::ServerError;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tracing::error;

pub mod secret;

pub type CryptoResult<T> = Result<T, CryptoError>;
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Password hashing error")]
    PasswordHashing(String),
}

impl From<CryptoError> for ServerError {
    fn from(value: CryptoError) -> Self {
        match value {
            CryptoError::PasswordHashing(msg) => {
                error!("Password hashing error: {msg}");
                ServerError::Unexpected
            }
        }
    }
}

pub fn generate_secret() -> Secret {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    Secret::new(hex::encode(bytes))
}

pub fn hash_secret(password: &Secret) -> CryptoResult<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.reveal_bytes(), &salt)
        .map_err(|err| CryptoError::PasswordHashing(err.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_secret(password: &Secret, hash: impl AsRef<str>) -> bool {
    let argon2 = Argon2::default();
    let Ok(password_hash) = PasswordHash::new(hash.as_ref()) else {
        return false;
    };

    argon2
        .verify_password(password.reveal_bytes(), &password_hash)
        .is_ok()
}
