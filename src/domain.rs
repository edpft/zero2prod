use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl TryFrom<String> for SubscriberName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_character = value.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_character {
            Err(format!("{value} is not a valid subscriber name."))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn new(email: String, name: SubscriberName) -> Self {
        Self { email, name }
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
