use derive_more::From;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Free-form email template text that may contain placeholder tokens.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::FromStr,
    From,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[from(String, &str)]
pub struct TemplatePart(String);

impl TemplatePart {
    /// Replace each known placeholder with its provided value.
    pub fn materialize_with(&self, replacements: &[(String, String)]) -> String {
        let mut raw = self.0.clone();

        for (placeholder, value) in replacements {
            raw = raw.replace(placeholder, value);
        }

        #[cfg(debug_assertions)]
        {
            let rng = "<RNG>";
            if raw.contains(rng) {
                let rnd: u64 = rand::random();
                raw = raw.replace(rng, rnd.to_string().as_str());
            }
        }

        raw
    }
}

impl Default for TemplatePart {
    fn default() -> Self {
        Self("Invoice <INV_NO> from <FROM_CO>".to_owned())
    }
}
