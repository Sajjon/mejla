use derive_more::{Deref, From};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

/// A valid email address.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    From,
    Deref,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[display("{}", _0)]
pub struct EmailAddress(lettre::Address);

impl EmailAddress {
    pub fn sample() -> Self {
        Self::sample_alice()
    }

    pub fn sample_other() -> Self {
        Self::sample_bob()
    }

    pub fn sample_alice() -> Self {
        Self::from_str("alice@example.com").expect("valid sample email")
    }

    pub fn sample_bob() -> Self {
        Self::from_str("bob@example.com").expect("valid sample email")
    }

    pub fn sample_carol() -> Self {
        Self::from_str("carol@example.com").expect("valid sample email")
    }

    pub fn sample_dave() -> Self {
        Self::from_str("dave@example.com").expect("valid sample email")
    }

    pub fn sample_erin() -> Self {
        Self::from_str("erin@example.com").expect("valid sample email")
    }
}
