use crate::{
    EmailAccount, EmailAddress, EmailSettingsSelector, EncryptedAppPassword,
    EncryptedEmailSettings, SmtpServer, Template,
};
use indexmap::IndexSet;
use secrecy::SecretString;

fn select_or_default<T, E, F>(
    selector: Option<EmailSettingsSelector>,
    target: EmailSettingsSelector,
    default: &T,
    builder: F,
) -> Result<T, E>
where
    F: FnOnce(&T) -> Result<T, E>,
    T: Clone,
{
    if selector
        .as_ref()
        .map(|s| s.includes(target))
        .unwrap_or(selector.is_none())
    {
        builder(default)
    } else {
        Ok(default.clone())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn ask_for_email<E>(
    default: EncryptedEmailSettings,
    data_selector: Option<EmailSettingsSelector>,
    config_render: impl FnOnce(),
    ask_for_password: impl FnOnce(bool, &str, &str) -> Result<SecretString, E>,
    ask_for_email_encryption_password_with_confirmation: impl FnOnce(bool) -> Result<SecretString, E>,
    ask_for_smtp_server: impl FnOnce(&SmtpServer) -> Result<SmtpServer, E>,
    ask_for_sender: impl FnOnce(&EmailAccount) -> Result<EmailAccount, E>,
    ask_for_template: impl FnOnce(&Template) -> Result<Template, E>,
    ask_for_reply_to: impl FnOnce(Option<&EmailAccount>) -> Result<Option<EmailAccount>, E>,
    ask_for_recipients: impl FnOnce(&IndexSet<EmailAddress>) -> Result<IndexSet<EmailAddress>, E>,
    ask_for_cc_recipients: impl FnOnce(&IndexSet<EmailAddress>) -> Result<IndexSet<EmailAddress>, E>,
    ask_for_bcc_recipients: impl FnOnce(&IndexSet<EmailAddress>) -> Result<IndexSet<EmailAddress>, E>,
    recipients_empty_error: impl FnOnce() -> E,
    on_built: impl FnOnce(&EncryptedEmailSettings),
) -> Result<EncryptedEmailSettings, E> {
    config_render();

    let is_editing_but_skip_secrets = data_selector
        .as_ref()
        .map(|s| !s.requires_encryption_password())
        .unwrap_or(false);

    let (salt, app_password_encrypted) = if is_editing_but_skip_secrets {
        (default.salt().clone(), default.smtp_app_password().clone())
    } else {
        let app_password_plaintext = ask_for_password(
            true,
            "SMTP App Password",
            "Used to authenticate sender account",
        )?;
        let salt = crate::Salt::generate();
        let encryption_password = ask_for_email_encryption_password_with_confirmation(true)?;
        let encryption_key = EncryptedAppPassword::new_by_deriving_and_encrypting(
            app_password_plaintext,
            encryption_password,
            &salt,
        );
        (salt, encryption_key)
    };

    let smtp_server = select_or_default(
        data_selector,
        EmailSettingsSelector::SmtpServer,
        default.smtp_server(),
        ask_for_smtp_server,
    )?;

    let sender = select_or_default(
        data_selector,
        EmailSettingsSelector::Sender,
        default.sender(),
        ask_for_sender,
    )?;

    let template = select_or_default(
        data_selector,
        EmailSettingsSelector::Template,
        default.template(),
        ask_for_template,
    )?;

    let reply_to = select_or_default(
        data_selector,
        EmailSettingsSelector::ReplyTo,
        default.reply_to(),
        |d| ask_for_reply_to(d.as_ref()),
    )?;

    let recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::Recipients,
        default.recipients(),
        ask_for_recipients,
    )?;

    if recipients.is_empty() {
        return Err(recipients_empty_error());
    }

    let cc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::CcRecipients,
        default.cc_recipients(),
        ask_for_cc_recipients,
    )?;

    let bcc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::BccRecipients,
        default.bcc_recipients(),
        ask_for_bcc_recipients,
    )?;

    let email_settings = EncryptedEmailSettings::builder()
        .sender(sender)
        .smtp_server(smtp_server)
        .smtp_app_password(app_password_encrypted)
        .maybe_reply_to(reply_to)
        .recipients(recipients.clone())
        .bcc_recipients(bcc_recipients)
        .cc_recipients(cc_recipients)
        .template(template)
        .salt(salt)
        .build();

    on_built(&email_settings);

    Ok(email_settings)
}
