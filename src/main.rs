use rand::random_range;
use rustyline::{ DefaultEditor, Result };
use colored::{ Colorize };

struct ScaleLib<'a> {
    all_notes: Vec<&'a str>,
    all_scales: Vec<&'a str>,
}

impl<'a> ScaleLib<'a> {
    fn new() -> ScaleLib<'a> {
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
                "Harmonic Mator",
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

    fn get_random_note(&self) -> String {
        let note_ind = random_range(0..self.all_notes.len());
        self.all_notes[note_ind].to_string()
    }
    fn get_random_scale(&self) -> String {
        let scale_ind = random_range(0..self.all_scales.len());
        self.all_scales[scale_ind].to_string()
    }
    fn get_notes_vec_length(&self) -> usize {
        self.all_notes.len()
    }
    fn get_scales_vec_length(&self) -> usize {
        self.all_scales.len()
    }
}

struct Scale {
    note_name: String,
    scale_name: String,
}

impl Scale {
    fn new(note: &str, scale: &str) -> Scale {
        Scale {
            note_name: note.to_string(),
            scale_name: scale.to_string(),
        }
    }
}

fn main() {
    driver();
    let _ = prompt();
}

fn prompt() -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    println!("{} {}/{}", "Try again?".italic(), "Y".green(), "N".red());
    let readline = rl.readline(">> ")?;
    if readline.trim().to_uppercase() == "Y" || readline.trim().to_uppercase() == "YES" {
        println!("Lets try this again...");
        let _ = main();
    } else {
        println!("Goodbye!");
    }
    Ok(readline)
}

fn driver() {
    let lib = ScaleLib::new();
    let note = lib.get_random_note();
    let scale = lib.get_random_scale();

    let s = Scale::new(&note, &scale);
    println!(
        "{} {} {}",
        "Why don't you try:".italic(),
        s.note_name.green().bold(),
        s.scale_name.green().bold()
    );
}

#[cfg(test)]
mod tests {
    use super::ScaleLib;
    // use super::Scale;
    #[test]
    fn basics() {
        let lib = ScaleLib::new();

        assert_eq!(lib.get_notes_vec_length(), 17);
        assert_eq!(lib.get_scales_vec_length(), 35);

        // TODO: Write more tests
    }

    #[test]
    fn good_val() {
        let lib = ScaleLib::new();

        assert_eq!(
            lib.all_notes.iter().any(|&all_notes| all_notes.contains("Bb")),
            true
        );
        assert_eq!(
            lib.all_scales.iter().any(|&all_scales| all_scales.contains("Messiaen 5")),
            true
        );
    }

    #[test]
    fn bad_val() {
        let lib = ScaleLib::new();
        assert_eq!(
            lib.all_notes.iter().any(|&all_notes| all_notes.contains("Cb")),
            false
        );

        assert_eq!(
            lib.all_scales.iter().any(|&all_scales| all_scales.contains("Doobitie Boo Bap")),
            false
        );
    }
}
