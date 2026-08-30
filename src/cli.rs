use clap::{Parser, Subcommand, ValueEnum};
#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short,long, help="generate scale with a random bpm; must start new session to change value", value_name="true|false")]
    pub bpm:bool,
    
    #[arg(long, short, value_enum)]
    pub mood: Option<Mood>,
}

impl Cli{
    pub fn run()->Cli{
        let args = Cli::parse();
        args
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mood{
    Happy,
    Sad,
    Dreamy,
    Mysterious
}

