use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// A binary attachment that can be added to an email.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Getters, Serialize, Deserialize)]
pub struct Attachment {
    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    mime_type: String,

    #[getset(get = "pub")]
    data: Vec<u8>,
}

impl Attachment {
    pub fn new(
        name: impl Into<String>,
        mime_type: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            name: name.into(),
            mime_type: mime_type.into(),
            data: data.into(),
        }
    }

    pub fn pdf(name: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self::new(name, "application/pdf", data)
    }

    pub fn sample() -> Self {
        Self::pdf("sample.pdf", vec![0xde, 0xad, 0xbe, 0xef])
    }

    pub fn sample_other() -> Self {
        Self::new("notes.txt", "text/plain", b"Hello from mejla".to_vec())
    }
}
