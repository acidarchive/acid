use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Time {
    Note,
    Tied,
    Rest,
}

impl AsRef<str> for Time {
    fn as_ref(&self) -> &str {
        match self {
            Time::Note => "note",
            Time::Tied => "tied",
            Time::Rest => "rest",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::patterns::Time;
    use serde_json;

    #[test]
    fn time_serialization() {
        let times = vec![Time::Note, Time::Tied, Time::Rest];

        for time in times {
            let json = serde_json::to_string(&time).unwrap();
            let deserialized: Time = serde_json::from_str(&json).unwrap();
            assert_eq!(time, deserialized);
        }
    }
}
