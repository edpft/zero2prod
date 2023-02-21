use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl TryFrom<String> for SubscriberEmail {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validate_email(&value) {
            let subscriber_email = Self(value);
            Ok(subscriber_email)
        } else {
            Err(format!("{value} is not a valid subscriber email."))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn invalid_emails_are_rejected() {
        let test_cases = vec![
            ("".to_string(), "an empty string"),
            ("ursuladomain.com".to_string(), "missing the @ symbol"),
            ("@domain.com".to_string(), "missing the local-part"),
        ];

        for (email, error_message) in test_cases {
            assert!(
                SubscriberEmail::try_from(email).is_err(),
                "Email wasn't rejected even though it was {}.",
                error_message
            )
        }
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) {
        assert!(SubscriberEmail::try_from(valid_email.0).is_ok())
    }
}
