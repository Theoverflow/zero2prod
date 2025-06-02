//! src/domain/suscriber_email.rs
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SubscriberEmail {
    #[validate(email(message = "must be a valid email address"))]
    pub email_field: String,
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.email_field.fmt(f)
    }
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let candidate = SubscriberEmail {
            email_field: s.clone(),
        };
        match candidate.validate() {
            Ok(()) => Ok(candidate),
            // Err(_) => Err(ValidationError::new("Wrong mail entry")),
            Err(_) => Err(format!("{} is not a valid subscriber email.", s)),
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email_field
    }
}
// TO CHECK LATER
// #[cfg(test)]
// mod tests {
//     // We have removed the `assert_ok` import.
//     use super::SubscriberEmail;
//     use fake::{Fake, faker::internet::en::SafeEmail};
//     // [...]
//     // Both `Clone` and `Debug` are required by `quickcheck`
//     #[derive(Debug, Clone)]
//     struct ValidEmailFixture(pub String);
//     impl quickcheck::Arbitrary for ValidEmailFixture {

//         fn arbitrary(g: &mut quickcheck::Gen) -> Self {
//             let email: String = SafeEmail().fake_with_rng(g);
//             ValidEmailFixture(email)
//         }

//         /// No shrinking for our fixture.
//         fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
//             Box::new(std::iter::empty())
//         }
//     }
//     #[quickcheck_macros::quickcheck]
//     fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
//         SubscriberEmail::parse(valid_email.0).is_ok()
//     }
// }
