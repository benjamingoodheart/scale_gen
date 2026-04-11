use colored::{ Colorize };
use inquire::Confirm;
mod cli;
mod scale;
mod bpm;

fn main() -> Result<(), String>{
    
    let args = cli::Cli::run();
    
    match args.bpm{
        true => {
            let b = bpm::BPM::new();
            let rand_bpm = b.get_random_bpm();
            driver(Some(rand_bpm));
        },
        false => {driver(None);}
    }
    let _ = prompt();
    

    Ok(())
}

fn prompt() -> Result<String, String> {
    let try_again = Confirm::new("Would you like to try another suggestion?").with_default(false).prompt();

    match try_again{
        Ok(true) => {println!("Great, going again..."); let _ = main();},
        Ok(false)=> println!("Cool, goodbye!"),
        Err(_) => println!("Try again")
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
    use clap::command;
    use clap::arg;
    use crate::bpm::BPM;
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

    #[test]
    fn clap_test_no_args(){
        command!().debug_assert();
    }
    #[test]
    fn clap_test_bpm_arg(){
        let _ = command!().arg(arg!(<BPM>).help("generate scale with a random bpm; must start new session to change value"));
    }

    #[test]
    fn random_bpm(){
        let b = BPM::new();
        let res = b.get_random_bpm();
        assert!(res >= b.floor_bpm);
        assert!(res<=b.ceiling_bpm);
    }

}
