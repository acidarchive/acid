use crate::domain::{Note, StepNumber, Time, Transpose};

pub struct NewTB303Step {
    pub number: StepNumber,
    pub note: Option<Note>,
    pub transpose: Option<Transpose>,
    pub time: Time,
    pub accent: Option<bool>,
    pub slide: Option<bool>,
}
