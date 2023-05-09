mod args;
mod chunk;
mod chunk_type;
mod png;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::Parser;
use std::fs::{read, read_to_string, File, OpenOptions};
use std::io::Write;
// mod commands;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Args::parse();

    match &cli.command {
        args::Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let data = read(&file_path).expect("Error reading input file");
            let mut png = Png::try_from(&data[..]).expect("Failed to decode png");

            let chunk_type = chunk_type.parse::<ChunkType>().expect("Invalid Chunk Type");
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            png.append_chunk(chunk);

            let file_to_write_to = output_file.clone().unwrap_or_else(|| file_path.clone());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_to_write_to)
                .expect("failed to open output file for writing");
            file.write_all(&png.as_bytes())
                .expect("Failed to write to output file");
        }
        args::Commands::Decode {
            file_path,
            chunk_type,
        } => {
            let data = read(&file_path).expect("Error reading input file");
            let png = Png::try_from(&data[..]).expect("Failed to decode png");

            if let Some(target_chunk) = png.chunk_by_type(chunk_type) {
                println!(
                    "Chunk data for {chunk_type}: {}",
                    target_chunk.data_as_string()?
                )
            } else {
                println!("Chunk type {chunk_type} not found");
            };
        }
        args::Commands::Remove {
            file_path,
            chunk_type,
        } => {}
        args::Commands::Print { file_path } => {}
    }
    Ok(())
}
