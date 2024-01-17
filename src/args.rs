use std::path::PathBuf;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    pub input_file: PathBuf,
    pub chunk_type: String,
    pub message: String,
}

pub struct DecodeArgs {
    pub input_file: PathBuf,
    pub chunk_type: String,
}

pub struct RemoveArgs {
    pub input_file: PathBuf,
    pub chunk_type: String,
}

pub struct PrintArgs {
    pub input_file: PathBuf,
}
