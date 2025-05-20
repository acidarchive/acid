use crate::domain::{Author, Description, Knob, NewTB303Step, Title, Waveform, BPM};

pub struct NewTB303Pattern {
    pub author: Option<Author>,
    pub title: Option<Title>,
    pub description: Option<Description>,
    pub waveform: Option<Waveform>,
    pub triplets: Option<bool>,
    pub bpm: Option<BPM>,
    pub cut_off_freq: Option<Knob>,
    pub resonance: Option<Knob>,
    pub env_mod: Option<Knob>,
    pub decay: Option<Knob>,
    pub accent: Option<Knob>,
    pub steps: Vec<NewTB303Step>,
}
