use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI{
    /// provide a random BPM
    #[arg(short, long, value_name = "TRUE|FALSE")]
    bpm: Option<bool>,
    
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}