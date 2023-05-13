use super::Result;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove {
        file_path: String,
        chunk_type: String,
    },
    Print {
        file_path: String,
    },
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Command::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            } => {
                let mut png = Png::from_file(file_path);

                let chunk_type = chunk_type.parse::<ChunkType>().expect("Invalid Chunk Type");
                let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

                png.append_chunk(chunk);

                let file_to_write_to = output_file.to_owned().unwrap_or(file_path.to_string());

                png.to_file(&file_to_write_to)
            }
            Command::Decode {
                file_path,
                chunk_type,
            } => {
                let png = Png::from_file(file_path);

                if let Some(target_chunk) = png.chunk_by_type(chunk_type) {
                    println!(
                        "Chunk data for {chunk_type}: {}",
                        target_chunk.data_as_string().unwrap()
                    )
                } else {
                    println!("Chunk type {chunk_type} not found");
                };
                Ok(())
            }
            Command::Remove {
                file_path,
                chunk_type,
            } => {
                let mut png = Png::from_file(file_path);
                let chunk = png.remove_chunk(chunk_type)?;
                png.to_file(file_path)?;
                println!("Removed chunk: {}", chunk);
                Ok(())
            }
            Command::Print { file_path } => {
                let png = Png::from_file(file_path);
                println!("{png}");
                Ok(())
            }
        }
    }
}
