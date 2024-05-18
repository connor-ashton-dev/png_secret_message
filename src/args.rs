use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "A tool for embedding secret messages in PNG files")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        #[arg(help = "Path to the PNG file")]
        file_path: PathBuf,
        #[arg(help = "Chunk type")]
        chunk_type: String,
        #[arg(help = "Message to encode")]
        message: String,
    },
    Decode {
        #[arg(help = "Path to the PNG file")]
        file_path: PathBuf,
        #[arg(help = "Chunk type")]
        chunk_type: String,
    },
    Remove {
        #[arg(help = "Path to the PNG file")]
        file_path: PathBuf,
        #[arg(help = "Chunk type")]
        chunk_type: String,
    },
    Print {
        #[arg(help = "Path to the PNG file")]
        file_path: PathBuf,
    },
}
