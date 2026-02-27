mod build_email_settings;
mod email_account;
mod email_address;
mod error;
mod password;
mod smtp_server;
mod template;
mod util;

pub use build_email_settings::ask_for_email;
pub use email_account::{ask_for_email_account, ask_for_email_account_skippable};
pub use email_address::{
    EmailAddressRole, ask_for_email_address, ask_for_email_address_skippable,
    ask_for_many_email_addresses,
};
pub use error::Error;
pub use password::{
    DEFAULT_EMAIL_ENCRYPTION_PASSWORD_ENV_VAR, ask_for_email_encryption_password_with_confirmation,
    ask_for_email_encryption_password_with_confirmation_in_env, ask_for_password,
    ask_for_password_once_with_length, get_email_encryption_password,
};
pub use smtp_server::ask_for_smtp_server;
pub use template::ask_for_template;
pub use util::format_help_skippable;

pub type Result<T, E = Error> = std::result::Result<T, E>;
