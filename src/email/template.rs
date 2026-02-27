use crate::TemplatePart;
use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// Template is a struct that contains the subject and body format for an email.
#[derive(Debug, Clone, Default, PartialEq, Eq, Builder, Getters, Serialize, Deserialize)]
pub struct Template {
    #[getset(get = "pub")]
    subject_format: TemplatePart,

    #[getset(get = "pub")]
    body_format: TemplatePart,
}

impl Template {
    pub fn materialize_with(&self, replacements: &[(String, String)]) -> (String, String) {
        let subject = self.subject_format.materialize_with(replacements);
        let body = self.body_format.materialize_with(replacements);
        (subject, body)
    }
}
