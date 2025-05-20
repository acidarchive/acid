#[derive(Debug)]
pub struct Octave(String);

impl Octave {
    pub fn parse(s: String) -> Result<Octave, String> {
        let is_not_valid_stem = !["up", "down"].contains(&s.as_str());

        if is_not_valid_stem {
            Err(format!(
                "{} is not a valid octave. Can only be one of 'up', 'down'",
                s
            ))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Octave {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Octave;
    use claims::{assert_err, assert_ok};

    #[test]
    fn invalid_octave_is_rejected() {
        let octave = "invalid_octave".to_string();
        assert_err!(Octave::parse(octave));
    }

    #[test]
    fn valid_octaves_are_accepted() {
        for stem in vec!["up", "down"] {
            assert_ok!(Octave::parse(stem.to_string()));
        }
    }
}
