use crate::crypto::secret::Secret;
use crate::message::server::ServerError;
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

#[cfg(feature = "argon2")]
pub fn generate_secret() -> Secret {
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::rand_core::RngCore;

    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    Secret::new(hex::encode(bytes))
}

#[cfg(feature = "argon2")]
pub fn hash_secret(password: &Secret) -> CryptoResult<String> {
    use argon2::PasswordHasher;
    use argon2::password_hash::SaltString;
    use argon2::password_hash::rand_core::OsRng;

    let argon2 = argon2::Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.reveal_bytes(), &salt)
        .map_err(|err| CryptoError::PasswordHashing(err.to_string()))?;
    Ok(hash.to_string())
}

#[cfg(feature = "argon2")]
pub fn verify_secret(password: &Secret, hash: impl AsRef<str>) -> bool {
    use argon2::PasswordVerifier;

    let argon2 = argon2::Argon2::default();
    let Ok(password_hash) = argon2::PasswordHash::new(hash.as_ref()) else {
        return false;
    };

    argon2
        .verify_password(password.reveal_bytes(), &password_hash)
        .is_ok()
}
