use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::PNG;
use crate::Result;

pub fn encode_message(file_path: &PathBuf, chunk_type: &str, message: &str) -> Result<()> {
    let mut png = get_png_from_file_path(file_path)?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data = message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);

    png.append_chunk(chunk);

    write_png_bytes_to_file(&png, file_path)?;

    Ok(())
}
pub fn decode_message(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let png = get_png_from_file_path(file_path)?;

    let chunk = match png.chunk_by_type(chunk_type) {
        Some(c) => c,
        None => return Err("No chunk found with data provided".into()),
    };

    let message = chunk.data_as_string()?;

    println!("{message}");

    Ok(())
}
pub fn remove_chunk(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let mut png = get_png_from_file_path(file_path)?;

    let removed_chunk = png.remove_chunk(chunk_type)?;

    write_png_bytes_to_file(&png, file_path)?;

    println!("Removed {:?} from {:?}", removed_chunk, file_path);

    Ok(())
}
pub fn print_chunks(file_path: &PathBuf) -> Result<()> {
    let png = get_png_from_file_path(file_path)?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}

fn write_png_bytes_to_file(png: &PNG, file_path: &PathBuf) -> Result<()> {
    let png_bytes = png.as_bytes();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&png_bytes)?;

    Ok(())
}

fn get_png_from_file_path(file_path: &PathBuf) -> Result<PNG> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let png = PNG::try_from(buffer.as_slice());
    png
}
