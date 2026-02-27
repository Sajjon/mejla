use thiserror::Error;

/// Errors that can occur while collecting email input through terminal prompts.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Passwords do not match")]
    PasswordDoesNotMatch,

    #[error(
        "Email password is too short, expected at least {min_length} characters, but found {actual_length}"
    )]
    EmailPasswordTooShort {
        min_length: usize,
        actual_length: usize,
    },

    #[error("Failed to parse email atom template: {underlying}")]
    EmailAtomTemplateError { underlying: String },

    #[error("Invalid email address for: {role}, because: {underlying}")]
    InvalidEmailAddress { role: String, underlying: String },

    #[error("Invalid name for email for: {role}, because: {underlying}")]
    InvalidNameForEmail { role: String, underlying: String },

    #[error("Invalid password for email {purpose}, because: {underlying}")]
    InvalidPasswordForEmail { purpose: String, underlying: String },

    #[error("Recipient addresses cannot be empty")]
    RecipientAddressesCannotBeEmpty,

    #[error("Failed to parse SMTP Server, because: {underlying}")]
    InvalidSmtpServer { underlying: String },
}

impl Error {
    pub fn email_atom_template_error(underlying: impl std::fmt::Display) -> Self {
        Self::EmailAtomTemplateError {
            underlying: underlying.to_string(),
        }
    }

    pub fn invalid_smtp_server(underlying: impl std::fmt::Display) -> Self {
        Self::InvalidSmtpServer {
            underlying: underlying.to_string(),
        }
    }

    pub fn invalid_email_address_for_role<E: std::fmt::Display>(
        role: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let role = role.to_string();
        move |e| Self::InvalidEmailAddress {
            role,
            underlying: e.to_string(),
        }
    }

    pub fn invalid_name_for_email_for_role<E: std::fmt::Display>(
        role: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let role = role.to_string();
        move |e| Self::InvalidNameForEmail {
            role,
            underlying: e.to_string(),
        }
    }

    pub fn invalid_password_for_email_purpose<E: std::fmt::Display>(
        purpose: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let purpose = purpose.to_string();
        move |e| Self::InvalidPasswordForEmail {
            purpose,
            underlying: e.to_string(),
        }
    }
}
