use log::info;
use rpassword::prompt_password;
use secrecy::{ExposeSecret as _, SecretString};

use super::{Error, Result};

pub const DEFAULT_EMAIL_ENCRYPTION_PASSWORD_ENV_VAR: &str = "EMAIL_ENCRYPTION_PASSWORD";
const PASSWORD_MIN_LENGTH: usize = 4;

fn validate(input: SecretString, min_length: usize) -> Result<SecretString> {
    let length = input.expose_secret().len();
    if length < min_length {
        Err(Error::EmailPasswordTooShort {
            min_length,
            actual_length: length,
        })
    } else {
        Ok(input)
    }
}

pub fn ask_for_password_once_with_length(
    prompt: &str,
    help: &str,
    min_length: usize,
    show_min_length: bool,
) -> Result<SecretString> {
    // Password from `read_password` will be zeroized at end of this function.
    let maybe_min_length_str = if show_min_length {
        format!(", min: #{} letters.", min_length)
    } else {
        "".to_owned()
    };
    prompt_password(format!("{} ({}{})", prompt, help, maybe_min_length_str))
        .map(SecretString::from)
        .map_err(Error::invalid_password_for_email_purpose(prompt))
        .and_then(|input| validate(input, min_length))
}

fn ask_for_password_once(prompt: &str, help: &str, show_min_length: bool) -> Result<SecretString> {
    ask_for_password_once_with_length(prompt, help, PASSWORD_MIN_LENGTH, show_min_length)
}

pub fn ask_for_password(with_confirmation: bool, prompt: &str, help: &str) -> Result<SecretString> {
    let first = ask_for_password_once(prompt, help, with_confirmation)?;
    if !with_confirmation {
        return Ok(first);
    }
    let second = ask_for_password_once("Confirm password", help, with_confirmation)?;
    if first.expose_secret() != second.expose_secret() {
        return Err(Error::PasswordDoesNotMatch);
    }
    Ok(first)
}

pub fn ask_for_email_encryption_password_with_confirmation_in_env(
    with_confirmation: bool,
    env_var: &str,
) -> Result<SecretString> {
    if let Ok(env_pw) = std::env::var(env_var) {
        if env_pw.len() >= PASSWORD_MIN_LENGTH {
            info!(
                "Read encryption password from ENV variable (`{}`) so skipping prompting of it",
                env_var
            );
            return Ok(SecretString::from(env_pw));
        }
    }
    ask_for_password(
        with_confirmation,
        "Encryption Password",
        "Used to encrypt the SMTP App Password",
    )
}

pub fn ask_for_email_encryption_password_with_confirmation(
    with_confirmation: bool,
) -> Result<SecretString> {
    ask_for_email_encryption_password_with_confirmation_in_env(
        with_confirmation,
        DEFAULT_EMAIL_ENCRYPTION_PASSWORD_ENV_VAR,
    )
}

pub fn get_email_encryption_password() -> Result<SecretString> {
    ask_for_email_encryption_password_with_confirmation(false)
}
