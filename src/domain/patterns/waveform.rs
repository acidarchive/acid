use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Waveform {
    Square,
    Sawtooth,
}

impl AsRef<str> for Waveform {
    fn as_ref(&self) -> &str {
        match self {
            Waveform::Square => "square",
            Waveform::Sawtooth => "sawtooth",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Waveform;
    use serde_json;

    #[test]
    fn waveform_serialization() {
        let waveforms = vec![Waveform::Square, Waveform::Sawtooth];

        for waveform in waveforms {
            let json = serde_json::to_string(&waveform).unwrap();
            let deserialized: Waveform = serde_json::from_str(&json).unwrap();
            assert_eq!(waveform, deserialized);
        }
    }
}
