#[derive(Debug)]
pub struct Note(String);

impl Note {
    const VALID_NOTES: [&'static str; 13] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "Chigh",
    ];

    pub fn parse(s: String) -> Result<Note, String> {
        if Self::VALID_NOTES.contains(&s.as_str()) {
            Ok(Self(s))
        } else {
            Err(format!(
                "{} is not a valid note. Can only be one of {}",
                s,
                Self::VALID_NOTES.join(", ")
            ))
        }
    }
}

impl AsRef<str> for Note {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn invalid_note_is_rejected() {
        let note = "L".to_string();
        assert_err!(Note::parse(note));
    }

    #[test]
    fn valid_notes_are_accepted() {
        for note in Note::VALID_NOTES {
            assert_ok!(Note::parse(note.to_string()));
        }
    }
}
