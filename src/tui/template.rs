use inquire::CustomType;

use crate::{Template, TemplatePart};

use super::{Error, Result};

fn ask_for_template_part(part: &str, default: &TemplatePart, tutorial: &str) -> Result<TemplatePart> {
    CustomType::<TemplatePart>::new(&format!("Email template for {}", part))
        .with_help_message(tutorial)
        .with_default(default.clone())
        .prompt()
        .map_err(Error::email_atom_template_error)
}

pub fn ask_for_template(default: &Template, tutorial: &str) -> Result<Template> {
    let subject = ask_for_template_part("subject", default.subject_format(), tutorial)?;
    let body = ask_for_template_part("body", default.body_format(), tutorial)?;
    Ok(Template::builder()
        .subject_format(subject)
        .body_format(body)
        .build())
}
