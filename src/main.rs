use clap::Parser;

use self::args::CLI;
use self::commands::{decode_message, encode_message, print_chunks, remove_chunk};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        args::Commands::Encode {
            file_path,
            chunk_type,
            message,
        } => encode_message(file_path, chunk_type, message),
        args::Commands::Decode {
            file_path,
            chunk_type,
        } => decode_message(file_path, chunk_type),
        args::Commands::Remove {
            file_path,
            chunk_type,
        } => remove_chunk(file_path, chunk_type),
        args::Commands::Print { file_path } => print_chunks(file_path),
    }
}
