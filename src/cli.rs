use clap::{Parser, Subcommand};

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
pub struct Cli{
    /// provide a random BPM
    #[arg(short, long, value_name = "TRUE|FALSE")]
    pub bpm: Option<bool>,
    //interactive commands
    #[command(subcommand)]
    command:Option<Commands>,
}
impl Cli{
    pub fn run()->Cli{
        let args = Cli::parse();
        args
    }
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Show a random bpm with suggestions
    Bpm_on,
    /// Do not show a random bpm with suggestions
    Bpm_off,
    /// Exit the program
    Exit,
}