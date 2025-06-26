use crate::domain::{Author, Description, Knob, Name, NewTB303Step, Tempo, Title, Waveform};

pub struct NewTB303Pattern {
    pub name: Name,
    pub author: Option<Author>,
    pub title: Option<Title>,
    pub description: Option<Description>,
    pub waveform: Option<Waveform>,
    pub triplets: Option<bool>,
    pub tempo: Option<Tempo>,
    pub tuning: Option<Knob>,
    pub cut_off_freq: Option<Knob>,
    pub resonance: Option<Knob>,
    pub env_mod: Option<Knob>,
    pub decay: Option<Knob>,
    pub accent: Option<Knob>,
    pub is_public: Option<bool>,
    pub steps: Vec<NewTB303Step>,
}
