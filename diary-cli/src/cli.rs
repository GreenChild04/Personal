use clap::{Parser, Subcommand};
use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(author=Config::AUTHOR, version=Config::VERSION, about=Config::ABOUT, long_about=Config::ABOUT)]
struct CliArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about="A simple test command for the cli")]
    Test
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Test => self.test()
        }
    }

    fn test(&self) {
        println!("Testing testing...");
        println!("Hello World!")
    }
}

static mut COUNT: u8 = 0; // Counts the amount of times cli is run (cannot be more than once!)

pub fn run() {
    // Safety checks
    unsafe { if COUNT > 0 { panic!("Error: cli cannot be run more than once during program lifetime!") } }
    unsafe { COUNT += 1; }
    let cli: CliArgs = CliArgs::parse();
    cli.command.execute();
}