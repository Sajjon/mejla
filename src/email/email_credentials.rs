use secrecy::{ExposeSecret, SecretString};

use crate::{DecryptedEmailSettings, EmailAccount, SmtpServer};
use bon::Builder;
use getset::Getters;

/// Credentials for an email account, typically used for sending emails via SMTP.
#[derive(Debug, Clone, Builder, Getters)]
pub struct EmailCredentials {
    #[builder(default)]
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[getset(get = "pub")]
    account: EmailAccount,

    /// The password for the email account, typically an "App Password".
    #[getset(get = "pub")]
    password: SecretString,
}

impl From<DecryptedEmailSettings> for EmailCredentials {
    fn from(settings: DecryptedEmailSettings) -> Self {
        EmailCredentials::builder()
            .account(
                EmailAccount::builder()
                    .name(settings.sender().name().clone())
                    .email(settings.sender().email().clone())
                    .build(),
            )
            .password(settings.smtp_app_password().clone())
            .smtp_server(settings.smtp_server().clone())
            .build()
    }
}

impl PartialEq for EmailCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.smtp_server == other.smtp_server
            && self.account == other.account
            && self.password.expose_secret() == other.password.expose_secret()
    }
}

impl EmailCredentials {
    pub fn sample() -> Self {
        Self::builder()
            .smtp_server(SmtpServer::default())
            .account(EmailAccount::sample_alice())
            .password("open sesame".into())
            .build()
    }

    pub fn sample_other() -> Self {
        Self::builder()
            .smtp_server(SmtpServer::default())
            .account(EmailAccount::sample_bob())
            .password("super_secret".into())
            .build()
    }
}
