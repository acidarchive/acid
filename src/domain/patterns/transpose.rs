use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Transpose {
    Up,
    Down,
}

impl AsRef<str> for Transpose {
    fn as_ref(&self) -> &str {
        match self {
            Transpose::Up => "up",
            Transpose::Down => "down",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Transpose;

    #[test]
    fn transpose_serialization() {
        let transposes = vec![Transpose::Up, Transpose::Down];

        for transpose in transposes {
            let json = serde_json::to_string(&transpose).unwrap();
            let deserialized: Transpose = serde_json::from_str(&json).unwrap();
            assert_eq!(transpose, deserialized);
        }
    }
}
