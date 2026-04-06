#![allow(warnings)]
use clap::crate_name;
use rustyline::{ DefaultEditor, Result };
use colored::{ Colorize };
use std::io::Write;
use std::io::{Error,ErrorKind};
use clap::{Parser, Subcommand};
use inquire::Confirm;
mod cli;
mod scale;
mod bpm;

fn main() -> Result<()>{
    
    let args = cli::Cli::run();
    
    match args.bpm{
        Some(true) => {
            let b = bpm::BPM::new();
            let rand_bpm = b.get_random_bpm();
            driver(Some(rand_bpm));
        },
        _ => {driver(None);}
    }
    let _ = prompt();
    

    Ok(())
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
    let try_again = Confirm::new("Would you like to another suggestion?").with_default(false).prompt();

    match try_again{
        Ok(true) => {println!("Great"); main();},
        Ok(false)=> println!("Cool, goodbye!"),
        Err(_) => println!("try again")
    }
    Ok("Ok".to_string())
}


fn driver(rand_bpm:Option<i32>) {
    let lib = scale::ScaleLib::new();
    let note = lib.get_random_note();
    let scale = lib.get_random_scale();

    let s = scale::Scale::new(&note, &scale);
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
    use crate::scale::ScaleLib;
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
