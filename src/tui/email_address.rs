use crate::EmailAddress;
use derive_more::Display;
use indexmap::IndexSet;
use inquire::{Confirm, CustomType};
use log::warn;

use super::{Error, Result, format_help_skippable};

#[derive(Display, Clone, Copy, Debug)]
pub enum EmailAddressRole {
    #[display("Reply-To")]
    ReplyTo,
    #[display("Sender")]
    Sender,
    #[display("Recipient")]
    Recipient,
    #[display("CC")]
    Cc,
    #[display("BCC")]
    Bcc,
}

pub fn ask_for_email_address_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAddress>,
) -> Result<Option<EmailAddress>> {
    let label = format!("{}'s email address?", role);
    let help = format_help_skippable(format!("Email address for {}", role));
    let prompt = CustomType::<EmailAddress>::new(&label).with_help_message(&help);

    let result = if let Some(default) = default {
        prompt.with_default(default.clone()).prompt_skippable()
    } else {
        prompt.prompt_skippable()
    };

    result.map_err(Error::invalid_email_address_for_role(role))
}

pub fn ask_for_email_address(role: EmailAddressRole, default: &EmailAddress) -> Result<EmailAddress> {
    CustomType::<EmailAddress>::new(&format!("{}'s email address?", role))
        .with_help_message(&format!("Email address for {}", role))
        .with_default(default.clone())
        .prompt()
        .map_err(Error::invalid_email_address_for_role(role))
}

pub fn ask_for_many_email_addresses(
    role: EmailAddressRole,
    defaults: &IndexSet<EmailAddress>,
) -> Result<IndexSet<EmailAddress>> {
    let mut emails = IndexSet::new();
    loop {
        let Some(email) = ask_for_email_address_skippable(role, defaults.get_index(emails.len()))?
        else {
            break;
        };
        if emails.contains(&email) {
            warn!("Email address already exists, skipping");
            continue;
        }
        emails.insert(email);
        let another = Confirm::new(&format!("Add another {} email address?", role))
            .with_default(true)
            .prompt()
            .unwrap_or(true);
        if !another {
            break;
        }
    }
    Ok(emails)
}
