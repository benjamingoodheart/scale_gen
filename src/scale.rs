use rand::random_range;
pub struct ScaleLib<'a> {
    pub all_notes: Vec<&'a str>,
    pub all_scales: Vec<&'a str>,
}

impl<'a> ScaleLib<'a> {
    pub fn new() -> ScaleLib<'a> {
        ScaleLib {
            all_notes: vec![
                "Ab",
                "A",
                "A#",
                "Bb",
                "B",
                "C",
                "C#",
                "Db",
                "D",
                "D#",
                "Eb",
                "E",
                "F",
                "F#",
                "Gb",
                "G",
                "G#"
            ],
            all_scales: vec![
                "Major",
                "Minor",
                "Dorian",
                "Mixolydian",
                "Lydian",
                "Phrygian",
                "Locrian",
                "Whole Tone",
                "Half-whole Dim.",
                "Whole-half Dim.",
                "Minor Blues",
                "Minor Pentatonic",
                "Major Pentatonic",
                "Harmonic Minor",
                "Harmonic Major",
                "Dorian #4",
                "Phrygian Dominant",
                "Melodic Minor",
                "Lydian Augmented",
                "Lydian Dominant",
                "Super Locrian",
                "8-Tone Spanish",
                "Bhairav",
                "Hungarian Minor",
                "Hirajoshi",
                "In-sen",
                "Iwato",
                "Kumor",
                "Pelog Selisir",
                "Peloa Tembung",
                "Messiaen 3",
                "Messiaen 4",
                "Messiaen 5",
                "Messiaen 6",
                "Messiaen 7"
            ],
        }
    }

    pub fn get_random_note(&self) -> String {
        let note_ind = random_range(0..self.all_notes.len());
        self.all_notes[note_ind].to_string()
    }
    pub fn get_random_scale(&self) -> String {
        let scale_ind = random_range(0..self.all_scales.len());
        self.all_scales[scale_ind].to_string()
    }
    pub fn get_notes_vec_length(&self) -> usize {
        self.all_notes.len()
    }
    pub fn get_scales_vec_length(&self) -> usize {
        self.all_scales.len()
    }
}

pub struct Scale {
    pub note_name: String,
    pub scale_name: String,
}

impl Scale {
    pub fn new(note: &str, scale: &str) -> Scale {
        Scale {
            note_name: note.to_string(),
            scale_name: scale.to_string(),
        }
    }
}
