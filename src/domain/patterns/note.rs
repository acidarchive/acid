use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq)]
pub enum Note {
    #[serde(rename = "C")]
    C,
    #[serde(rename = "C#")]
    CSharp,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "D#")]
    DSharp,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "F#")]
    FSharp,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "G#")]
    GSharp,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "A#")]
    ASharp,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "Chigh")]
    Chigh,
}

impl AsRef<str> for Note {
    fn as_ref(&self) -> &str {
        match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
            Note::Chigh => "Chigh",
        }
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::patterns::Note;
    use serde_json;

    #[test]
    fn note_serialization() {
        let notes = vec![
            Note::C,
            Note::CSharp,
            Note::D,
            Note::DSharp,
            Note::E,
            Note::F,
            Note::FSharp,
            Note::G,
            Note::GSharp,
            Note::A,
            Note::ASharp,
            Note::B,
            Note::Chigh,
        ];

        for note in notes {
            let json = serde_json::to_string(&note).unwrap();
            let deserialized: Note = serde_json::from_str(&json).unwrap();
            assert_eq!(note, deserialized);
        }
    }
}
