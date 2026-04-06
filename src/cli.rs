use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, help="generate scale with a random bpm", value_name="true|false")]
    pub bpm:Option<bool>
}
impl Cli{
    pub fn run()->Cli{
        let args = Cli::parse();
        args
    }
}
