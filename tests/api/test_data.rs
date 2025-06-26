use serde_json::json;

pub fn get_valid_tb303_pattern_data(is_public: Option<bool>) -> String {
    let is_public = is_public.unwrap_or(false);

    json!({
        "name": "Pattern 1",
        "author": "Humanoind",
        "title": "Stakker humanoid",
        "description": "This is a demo pattern for the TB-303. It's a classic acid house pattern.",
        "is_public": is_public,
        "waveform": "sawtooth",
        "triplets": true,
        "tempo": 130,
        "cut_off_freq": 10,
        "resonance": 20,
        "env_mod": 30,
        "decay": 40,
        "accent": 50,
        "tuning": 60,
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
                "transpose": "down",
                "time": "note"
            },
            {
                "number": 4,
                "time": "tied"
            },
            {
                "number": 5,
                "note": "B",
                "transpose": "down",
                "time": "note",
                "slide": true
            },
            {
                "number": 6,
                "note": "B",
                "transpose": "down",
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
                "transpose": "down",
                "time": "note"
            },
            {
                "number": 9,
                "note": "D",
                "transpose": "down",
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
                "transpose": "down",
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
                "transpose": "down",
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
    })
    .to_string()
}
