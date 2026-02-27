use crate::EmailAccount;
use inquire::Text;

use super::{
    EmailAddressRole, Error, Result, ask_for_email_address, ask_for_email_address_skippable,
    format_help_skippable,
};

pub fn ask_for_email_account(role: EmailAddressRole, default: &EmailAccount) -> Result<EmailAccount> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format!("Will show up as the {} name", role))
        .with_default(default.name())
        .prompt()
        .map_err(Error::invalid_name_for_email_for_role(role))?;
    let email = ask_for_email_address(role, default.email())?;
    Ok(EmailAccount::builder().name(name).email(email).build())
}

pub fn ask_for_email_account_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAccount>,
) -> Result<Option<EmailAccount>> {
    let label = format!("Email account {} name?", role);
    let help = format_help_skippable(format!("Will show up as the {} name", role));
    let prompt = Text::new(&label).with_help_message(&help);

    let name = if let Some(default) = default {
        prompt.with_default(default.name()).prompt_skippable()
    } else {
        prompt.prompt_skippable()
    }
    .map_err(Error::invalid_name_for_email_for_role(role))?;

    let Some(name) = name else { return Ok(None) };

    let Some(email) = ask_for_email_address_skippable(role, default.map(|d| d.email()))? else {
        return Ok(None);
    };

    Ok(Some(
        EmailAccount::builder().name(name).email(email).build(),
    ))
}
