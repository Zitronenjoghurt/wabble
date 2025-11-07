use crate::crypto::secret::Secret;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub mod secret;

pub type CryptoResult<T> = Result<T, CryptoError>;
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Password hashing error")]
    PasswordHashing,
}

pub fn hash_password(password: &Secret) -> CryptoResult<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.reveal_bytes(), &salt)
        .map_err(|_| CryptoError::PasswordHashing)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &Secret, hash: impl AsRef<str>) -> bool {
    let argon2 = Argon2::default();
    let Ok(password_hash) = PasswordHash::new(hash.as_ref()) else {
        return false;
    };

    argon2
        .verify_password(password.reveal_bytes(), &password_hash)
        .is_ok()
}
