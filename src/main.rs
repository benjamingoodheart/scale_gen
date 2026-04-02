#![allow(warnings)]
use rand::random_range;
use rustyline::{ DefaultEditor, Result };
use colored::{ Colorize };
mod cli;

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

struct BPM {
    floor_bpm: i32,
    ceiling_bpm: i32,
}

impl BPM {
    fn new()->BPM{
        BPM{
            floor_bpm: 60,
            ceiling_bpm: 225,
        }
    }
    fn get_random_bpm(&self)->i32{
        let my_bpm = random_range(self.floor_bpm..self.ceiling_bpm);
        my_bpm
    }
    fn set_floor(&mut self,floor_val: i32)->Result<()>{
        //TODO: add guard rails for higher than ceiling
        self.floor_bpm = floor_val;
        Ok(())
    }
    fn set_ceiling(&mut self, ceiling_val: i32)->Result<()>{
       //TODO: add guard rails for lower than floor
        self.ceiling_bpm = ceiling_val;
        Ok(())
    }
}

fn main() {
    let args = cli::Cli::run();
    
    match args.bpm{
        Some(true) => {
            let b = BPM::new();
            let rand_bpm = b.get_random_bpm();
            driver(Some(rand_bpm));
        },
        _ => {driver(None);}
    }
    let _ = prompt();
}

fn prompt() -> Result<String> {
    /**
    let mut rl = DefaultEditor::new()?;
    println!("{} {}/{}", "Try again?".italic(), "Y".green(), "N".red());
    let readline = rl.readline(">> ")?;
    match *readline.trim().to_uppercase(){
        "Y" => {println!("Let's try this again..."); let _ = main();},
        _ => println!("Goodbye!")
    }

    Ok(readline)
    */
    Ok("Ok".to_string())
}

fn driver(rand_bpm:Option<i32>) {
    let lib = ScaleLib::new();
    let note = lib.get_random_note();
    let scale = lib.get_random_scale();

    let s = Scale::new(&note, &scale);
    let has_bpm = rand_bpm.is_some();
    if has_bpm == true  {
        println!(
            "{} {} {} {} {} {}",
            "Why don't you try:".italic(),
            s.note_name.green().bold(),
            s.scale_name.green().bold(),
            "at".italic(),
            rand_bpm.expect("Invalid number").to_string().cyan().bold(),
            "bpm?".cyan().bold(),
        );
    }    
    if has_bpm == false{
        println!(
            "{} {} {}{}",
            "Why don't you try:".italic(),
            s.note_name.green().bold(),
            s.scale_name.green().bold(),
            "?".green().bold()
        );
    }

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
