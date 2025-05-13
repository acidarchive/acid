pub fn get_valid_tb303_pattern_data() -> String {
    r#"{
        "author": "Humanoind",
        "title": "Stakker humanoid",
        "description": "This is a demo pattern for the TB-303. It's a classic acid house pattern.",
        "waveform": "sawtooth",
        "triplets": true,
        "bpm": 130,
        "cut_off_freq": 10,
        "resonance": 20,
        "env_mod": 30,
        "decay": 40,
        "accent": 50,
        "steps": [
            {
                "number": 1,
                "note": "D",
                "time": "note"
            },
            {
                "number": 2,
                "note": "D",
                "time": "note"
            },
            {
                "number": 3,
                "note": "B",
                "octave": "down",
                "time": "note"
            },
            {
                "number": 4,
                "time": "tied"
            },
            {
                "number": 5,
                "note": "B",
                "octave": "down",
                "time": "note",
                "slide": true
            },
            {
                "number": 6,
                "note": "B",
                "octave": "down",
                "time": "note",
                "accent": true,
                "slide": true
            },
            {
                "number": 7,
                "time": "tied"
            },
            {
                "number": 8,
                "note": "B",
                "ocatve": "down",
                "time": "note"
            },
            {
                "number": 9,
                "note": "D",
                "octave": "down",
                "time": "note"
            },
            {
                "number": 10,
                "note": "D",
                "time": "note"
            },
            {
                "number": 11,
                "note": "B",
                "octave": "down",
                "time": "note"
            },
            {
                "number": 12,
                "note": "D",
                "time": "note"
            },
            {
                "number": 13,
                "time": "tied"
            },
            {
                "number": 14,
                "note": "B",
                "octave": "down",
                "time": "note"
            },
            {
                "number": 15,
                "note": "F",
                "time": "note"
            },
            {
                "number": 16,
                "time": "tied"
            }
        ]
    }
    "#
    .to_string()
}
