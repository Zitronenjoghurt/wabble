use bincode::{Decode, Encode};

pub type ValidationResult<T> = Result<T, ValidationError>;
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, thiserror::Error)]
pub enum ValidationError {
    #[error("Invite code is invalid")]
    InviteCodeFormat,
    #[error("Password exceeds length, must be less than or equal 128 characters")]
    PasswordExceedsLength,
    #[error("Password is too short, must be at least 8 characters")]
    PasswordTooShort,
    #[error(
        "Username contains invalid characters, must only contain alphanumeric characters or underscores"
    )]
    UsernameContainsInvalidCharacters,
    #[error("Username exceeds length, must be less than or equal 20 characters")]
    UsernameExceedsLength,
    #[error("Username is too short, must be at least 3 characters")]
    UsernameTooShort,
}

pub fn validate_username(username: &str) -> ValidationResult<()> {
    if username.len() > 20 {
        return Err(ValidationError::UsernameExceedsLength);
    };

    if username.len() < 3 {
        return Err(ValidationError::UsernameTooShort);
    }

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::UsernameContainsInvalidCharacters);
    };

    Ok(())
}

pub fn validate_password(password: &str) -> ValidationResult<()> {
    if password.len() > 128 {
        return Err(ValidationError::PasswordExceedsLength);
    } else if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }
    Ok(())
}

pub fn validate_invite_code(code: &str) -> ValidationResult<()> {
    uuid::Uuid::parse_str(code).map_err(|_| ValidationError::InviteCodeFormat)?;
    Ok(())
}
