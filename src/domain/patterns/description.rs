use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Description(String);

impl Description {
    pub fn parse(s: String) -> Result<Description, String> {
        let is_too_long = s.graphemes(true).count() > 500;
        let is_empty = s.trim().is_empty();

        if is_too_long || is_empty {
            Err(format!("{} is not a valid pattern description.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Description;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_500_grapheme_long_description_is_valid() {
        let description = "a".repeat(500);
        assert_ok!(Description::parse(description));
    }
    #[test]
    fn a_description_longer_than_500_graphemes_is_rejected() {
        let description = "a".repeat(501);
        assert_err!(Description::parse(description));
    }
    #[test]
    fn a_valid_description_is_parsed_successfully() {
        let note = "This is a pattern description".to_string();
        assert_ok!(Description::parse(note));
    }
}
