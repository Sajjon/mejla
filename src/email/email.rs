use crate::{Attachment, EmailAccount, EmailAddress};
use bon::Builder;
use getset::Getters;
use indexmap::IndexSet;

/// An email message that can be sent using an SMTP server.
#[derive(Debug, Clone, Builder, Getters, PartialEq)]
pub struct Email {
    #[builder(default)]
    #[getset(get = "pub")]
    public_recipients: IndexSet<EmailAddress>,

    #[builder(default)]
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    #[builder(default)]
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,

    #[builder(default)]
    #[getset(get = "pub")]
    subject: String,

    body: Option<String>,

    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    #[builder(default)]
    #[getset(get = "pub")]
    attachments: IndexSet<Attachment>,
}

impl Email {
    pub fn body(&self) -> String {
        self.body.clone().unwrap_or_default()
    }

    pub fn sample() -> Self {
        Self::builder()
            .public_recipients(IndexSet::from_iter(vec![EmailAddress::sample_bob()]))
            .cc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_erin()]))
            .subject("Sample Email Subject".to_string())
            .body("This is a sample email body.".to_string())
            .attachments(IndexSet::from_iter(vec![Attachment::sample()]))
            .build()
    }

    pub fn sample_other() -> Self {
        Self::builder()
            .public_recipients(IndexSet::from_iter(vec![EmailAddress::sample_alice()]))
            .cc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_dave()]))
            .subject("Another Sample Email Subject".to_string())
            .body("This is another sample email body.".to_string())
            .attachments(IndexSet::from_iter(vec![Attachment::sample_other()]))
            .build()
    }
}
