use secrecy::SecretString;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    Attachment, Email, EmailAccount, EmailAddress, EmailCredentials, EncryptedAppPassword,
    EncryptionKey, PbHkdfSha256, Salt, SmtpServer, Template,
};
use bon::Builder;
use getset::{Getters, WithSetters};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

pub type DecryptedEmailSettings = EmailSettings<SecretString>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

/// Represents settings for sending emails.
#[derive(
    derive_more::Debug, Clone, PartialEq, Eq, Builder, Getters, WithSetters, Serialize, Deserialize,
)]
pub struct EmailSettings<AppPassword: Zeroize> {
    #[getset(get = "pub")]
    #[debug("omitted")]
    smtp_app_password: AppPassword,

    #[getset(get = "pub")]
    #[debug("omitted")]
    salt: Salt,

    #[getset(get = "pub")]
    template: Template,

    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[getset(get = "pub", set_with = "pub")]
    sender: EmailAccount,

    #[getset(get = "pub")]
    recipients: IndexSet<EmailAddress>,

    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,
}

impl<AppPassword: Zeroize> Zeroize for EmailSettings<AppPassword> {
    fn zeroize(&mut self) {
        self.smtp_app_password.zeroize();
        self.salt.zeroize();
    }
}

impl<AppPassword: Zeroize> Drop for EmailSettings<AppPassword> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl<AppPassword: Zeroize> ZeroizeOnDrop for EmailSettings<AppPassword> {}

impl DecryptedEmailSettings {
    pub fn compose(
        &self,
        subject: impl Into<String>,
        body: impl Into<String>,
        attachments: IndexSet<Attachment>,
    ) -> (Email, EmailCredentials) {
        let email = Email::builder()
            .subject(subject.into())
            .body(body.into())
            .maybe_reply_to(self.reply_to().clone())
            .public_recipients(self.recipients().clone())
            .cc_recipients(self.cc_recipients().clone())
            .bcc_recipients(self.bcc_recipients().clone())
            .attachments(attachments)
            .build();
        let credentials = EmailCredentials::from(self.clone());
        (email, credentials)
    }

    pub fn sample() -> Self {
        Self::builder()
            .smtp_app_password(SecretString::from("encryption password"))
            .salt(Salt::sample())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample())
            .recipients(IndexSet::from([
                EmailAddress::sample_alice(),
                EmailAddress::sample_bob(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_dave(),
                EmailAddress::sample_erin(),
            ]))
            .build()
    }

    pub fn sample_other() -> Self {
        Self::builder()
            .smtp_app_password(SecretString::from("another encryption password"))
            .salt(Salt::sample_other())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample_other())
            .recipients(IndexSet::from([
                EmailAddress::sample_bob(),
                EmailAddress::sample_carol(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_dave()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_erin(),
                EmailAddress::sample_alice(),
            ]))
            .build()
    }
}

impl EncryptedEmailSettings {
    fn derive_and_decrypt_smtp_app_password(
        &self,
        encryption_key: EncryptionKey,
    ) -> crate::CryptoResult<DecryptedEmailSettings> {
        let decrypted = self.smtp_app_password.decrypt(encryption_key)?;
        Ok(DecryptedEmailSettings::builder()
            .smtp_app_password(decrypted)
            .maybe_reply_to(self.reply_to.clone())
            .smtp_server(self.smtp_server.clone())
            .sender(self.sender.clone())
            .recipients(self.recipients.clone())
            .cc_recipients(self.cc_recipients.clone())
            .bcc_recipients(self.bcc_recipients.clone())
            .template(self.template.clone())
            .salt(self.salt().clone())
            .build())
    }

    pub fn decrypt_smtp_app_password(
        &self,
        encryption_password: SecretString,
    ) -> crate::CryptoResult<DecryptedEmailSettings> {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password, self.salt());
        self.derive_and_decrypt_smtp_app_password(encryption_key)
    }

    pub fn sample() -> Self {
        Self::builder()
            .smtp_app_password(EncryptedAppPassword::sample())
            .salt(Salt::sample())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample())
            .recipients(IndexSet::from([
                EmailAddress::sample_alice(),
                EmailAddress::sample_bob(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_dave(),
                EmailAddress::sample_erin(),
            ]))
            .build()
    }

    pub fn sample_other() -> Self {
        Self::builder()
            .smtp_app_password(EncryptedAppPassword::sample_other())
            .salt(Salt::sample_other())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample_other())
            .recipients(IndexSet::from([
                EmailAddress::sample_bob(),
                EmailAddress::sample_carol(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_dave()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_erin(),
                EmailAddress::sample_alice(),
            ]))
            .build()
    }
}
