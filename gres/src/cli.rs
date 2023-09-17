use clap::*;
use crate::*;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about="A mere test command")]
    Test,
    #[command(about="Compresses a directory / file.", alias="c" )]
    Compress {
        #[arg(short, long, help="Sets if it removes the uncompressed file / directory")]
        remove: bool,
        #[arg(index=1, required=true, help="The path to the file / directory")]
        file_path: String,
        #[arg(short, long, help="The name of newly created compressed archive")]
        out_path: Option<String>,
    },
    #[command(about="Extracts a file / directory from a Gres Compressed Archive.", alias="e")]
    Extract {
        #[arg(short='t', long, help="Sets if it is a compressed directory or not")]
        is_tar: bool,
        #[arg(short, long, help="Sets if it removes the compressed archive")]
        remove: bool,
        #[arg(index=1, required=true, help="The path to the file / directory")]
        file_path: String,
        #[arg(index=2, required=true, help="The name of the extracted file / directory")]
        out_path: String,
    },
}

impl Commands {
    pub fn execute(self) {
        use Commands::*;
        match self {
            Test => println!("Hello, world!"),
            Compress { remove, file_path, out_path } => compress(file_path, out_path, remove),
            Extract { is_tar, remove, file_path, out_path } => decompress(file_path, out_path, is_tar, remove),
        }
    }
}


pub fn run() {
    Cli::parse().command.execute()
}