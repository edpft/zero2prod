use actix_web::web;

use crate::{
    domain::{subscriber_email::SubscriberEmail, subscriber_name::SubscriberName},
    routes::FormData,
};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn new(email: SubscriberEmail, name: SubscriberName) -> Self {
        Self { email, name }
    }
}

impl TryFrom<web::Form<FormData>> for NewSubscriber {
    type Error = String;

    fn try_from(value: web::Form<FormData>) -> Result<Self, Self::Error> {
        let subscriber_name = SubscriberName::try_from(value.0.name)?;
        let subscriber_email = SubscriberEmail::try_from(value.0.email)?;
        let new_subscriber = NewSubscriber::new(subscriber_email, subscriber_name);
        Ok(new_subscriber)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "è".repeat(256);
        assert!(SubscriberName::try_from(name).is_ok())
    }

    #[test]
    fn a_257_grapheme_long_name_is_rejected() {
        let name = "è".repeat(257);
        assert!(SubscriberName::try_from(name).is_err())
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert!(SubscriberName::try_from(name).is_err())
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert!(SubscriberName::try_from(name).is_err())
    }

    #[test]
    fn names_containing_an_invalid_characters_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert!(SubscriberName::try_from(name).is_err())
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert!(SubscriberName::try_from(name).is_ok())
    }
}
