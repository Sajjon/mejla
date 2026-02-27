use std::fmt::{Display, Formatter};

use bon::Builder;
use getset::Getters;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};
use secrecy::ExposeSecret;

use crate::{Attachment, Email, EmailAccount, EmailAddress, EmailCredentials};

/// Ephemeral helper struct to hold an email and sender while building `lettre::Message`.
#[derive(Debug, Clone, Builder, Getters)]
pub struct EmailWithSender {
    #[getset(get = "pub")]
    email: Email,

    #[getset(get = "pub")]
    sender: EmailAccount,
}

#[derive(Debug)]
pub enum SendEmailError {
    CreateEmail(lettre::error::Error),
    CreateSmtpTransport(lettre::transport::smtp::Error),
    SendEmail(lettre::transport::smtp::Error),
}

impl Display for SendEmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateEmail(error) => {
                write!(f, "failed to create email message: {error}")
            }
            Self::CreateSmtpTransport(error) => {
                write!(f, "failed to create SMTP transport: {error}")
            }
            Self::SendEmail(error) => {
                write!(f, "failed to send email: {error}")
            }
        }
    }
}

impl std::error::Error for SendEmailError {}

pub fn send_email_with_credentials(
    email: Email,
    credentials: EmailCredentials,
) -> Result<Response, SendEmailError> {
    let email_with_sender = EmailWithSender::builder()
        .email(email)
        .sender(credentials.account().clone())
        .build();
    let email = Message::try_from(email_with_sender).map_err(SendEmailError::CreateEmail)?;

    let smtp_server = credentials.smtp_server().clone();
    let creds = Credentials::from(credentials);

    let mailer = SmtpTransport::relay(smtp_server.as_ref())
        .map_err(SendEmailError::CreateSmtpTransport)?
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(SendEmailError::SendEmail)
}

trait CommonContentType: Sized {
    fn octet_stream() -> Self;
}

impl CommonContentType for ContentType {
    fn octet_stream() -> Self {
        ContentType::parse("application/octet-stream").expect("valid content type")
    }
}

impl From<Attachment> for SinglePart {
    fn from(attachment: Attachment) -> Self {
        let content_type = ContentType::parse(attachment.mime_type())
            .unwrap_or_else(|_| ContentType::octet_stream());

        lettre::message::Attachment::new(attachment.name().clone())
            .body(attachment.data().clone(), content_type)
    }
}

impl From<EmailAddress> for lettre::Address {
    fn from(address: EmailAddress) -> Self {
        (*address).clone()
    }
}

impl TryFrom<EmailWithSender> for Message {
    type Error = lettre::error::Error;

    fn try_from(email_with_sender: EmailWithSender) -> std::result::Result<Self, Self::Error> {
        let sender = email_with_sender.sender();
        let email = email_with_sender.email();
        let mut builder = Message::builder()
            .from(Mailbox::new(
                Some(sender.name().clone()),
                sender.email().clone().into(),
            ))
            .subject(email.subject().clone());

        if let Some(reply_to) = email.reply_to() {
            builder = builder.reply_to(Mailbox::new(
                Some(reply_to.name().clone()),
                reply_to.email().clone().into(),
            ));
        }

        for recipient in email.public_recipients() {
            builder = builder.to(Mailbox::new(None, recipient.clone().into()));
        }

        for recipient in email.cc_recipients() {
            builder = builder.cc(Mailbox::new(None, recipient.clone().into()));
        }

        for recipient in email.bcc_recipients() {
            builder = builder.bcc(Mailbox::new(None, recipient.clone().into()));
        }

        let attachments = email.attachments().clone();
        if attachments.is_empty() {
            builder.body(email.body())
        } else {
            let mut multipart = MultiPart::mixed()
                .singlepart(SinglePart::plain(email.body()))
                .singlepart(SinglePart::plain("\n".to_owned()));

            for attachment in attachments {
                multipart = multipart.singlepart(attachment.into());
            }

            builder.multipart(multipart)
        }
    }
}

impl From<EmailCredentials> for Credentials {
    fn from(credentials: EmailCredentials) -> Self {
        Credentials::new(
            credentials.account().email().to_string(),
            credentials.password().expose_secret().to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Attachment;

    #[test]
    fn pdf_content_type_is_applied() {
        let single_part: SinglePart = Attachment::pdf("a.pdf", vec![1, 2, 3]).into();
        let formatted = String::from_utf8(single_part.formatted()).expect("utf8 email headers");
        assert!(formatted.contains("application/pdf"));
    }
}
