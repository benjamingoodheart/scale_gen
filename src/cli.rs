use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short,long, help="generate scale with a random bpm; must start new session to change value", value_name="true|false")]
    pub bpm:bool
}
impl Cli{
    pub fn run()->Cli{
        let args = Cli::parse();
        args
    }
}
