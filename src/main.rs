use std::{
    fs,
    io::{Read, Write},
    str::FromStr,
};

use chunk::Chunk;
use chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
// mod commands;
pub mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // Open the original PNG file in read-write mode
    let file_path = "pic.png";
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true) // Open for both reading and writing
        .open(file_path)?;

    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    // Create a mutable PNG object from the buffer
    let mut my_png = png::Png::try_from(&buffer[..])?;

    // Create a new chunk and append it to the PNG
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let data: Vec<u8> = "This is where your secret message will be!"
        .bytes()
        .collect();
    let chunk = Chunk::new(chunk_type, data);
    my_png.append_chunk(chunk);

    // Write the modified PNG data back to the same file
    file.write_all(&my_png.as_bytes())?;

    // Truncate any remaining data (if the new PNG data is smaller)
    file.set_len(my_png.as_bytes().len() as u64)?;

    // Ensure the file is flushed and closed
    file.flush()?;

    println!("Changes written to 'pic.png'");

    let found_chunk = my_png.chunk_by_type("RuSt").unwrap();
    let mut data = found_chunk.get_data();

    let mut content: String = String::new();
    data.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())
}
