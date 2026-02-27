use crate::EmailAddress;
use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// A named sender and an email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Getters, Serialize, Deserialize)]
pub struct EmailAccount {
    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    email: EmailAddress,
}

impl EmailAccount {
    pub fn sample() -> Self {
        Self::sample_alice()
    }

    pub fn sample_other() -> Self {
        Self::sample_bob()
    }

    pub fn sample_alice() -> Self {
        Self::builder()
            .name("Alice Smith".to_string())
            .email(EmailAddress::sample_alice())
            .build()
    }

    pub fn sample_bob() -> Self {
        Self::builder()
            .name("Bob Johnson".to_string())
            .email(EmailAddress::sample_bob())
            .build()
    }
}
