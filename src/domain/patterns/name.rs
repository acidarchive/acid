use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Name(String);

impl Name {
    pub fn parse(s: String) -> Result<Name, String> {
        let is_too_long = s.graphemes(true).count() > 50;
        let is_empty = s.trim().is_empty();

        if is_too_long || is_empty {
            Err(format!("{s} is not a valid name."))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Name;
    use claims::{assert_err, assert_ok};

    #[test]
    fn an_empty_name_is_rejected() {
        let name = "".to_string();
        assert_err!(Name::parse(name));
    }

    #[test]
    fn a_name_with_only_whitespace_is_rejected() {
        let name = " ".to_string();
        assert_err!(Name::parse(name));
    }

    #[test]
    fn a_50_grapheme_long_name_is_valid() {
        let name = "a".repeat(50);
        assert_ok!(Name::parse(name));
    }
    #[test]
    fn a_name_longer_than_50_graphemes_is_rejected() {
        let name = "a".repeat(51);
        assert_err!(Name::parse(name));
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "myself".to_string();
        assert_ok!(Name::parse(name));
    }
}
