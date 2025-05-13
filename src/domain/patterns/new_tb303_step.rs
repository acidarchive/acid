use crate::domain::{Note, Octave, StepNumber, Time};

pub struct NewTB303Step {
    pub number: StepNumber,
    pub note: Option<Note>,
    pub octave: Option<Octave>,
    pub time: Time,
    pub accent: Option<bool>,
    pub slide: Option<bool>,
}
