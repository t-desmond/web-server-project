use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

pub struct CompressFile<'a> {
    file: &'a Path,
}

impl<'a> CompressFile<'a> {
    pub fn new(file_path: &'a Path) -> Self {
        Self { file: file_path }
    }

    pub fn compress(&self) -> Result<File, Box<dyn Error>> {
        let mut input_file = match File::open(&self.file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("couldn't open file: {}", e);
                return Err(Box::new(e));
            }
        };
        let mut buffer = Vec::new();

        match input_file.read_to_end(&mut buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("couldn't write to buffer: {}", e);
                return Err(Box::new(e));
            }
        };

        let mut compress = ZlibEncoder::new(Vec::new(), Compression::default());

        match compress.write_all(&buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("failed to write bytes to buffer: {}", e);
            }
        };

        let compressed_bytes = match compress.finish() {
            Ok(compressed_bytes) => compressed_bytes,
            Err(e) => {
                eprintln!("failed to write to buffer: {}", e);
                return Err(Box::new(e));
            }
        };

        match create_dir_all("compressed") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to create directory: {}", e);
            }
        };

        let compressed_file_name = format!(
            "compressed/{}.zlib",
            &self
                .file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        );
        let mut compressed_file = match File::create(compressed_file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!(
                    "failed to create destination file for compressed data: {}",
                    e
                );
                return Err(Box::new(e));
            }
        };

        match compressed_file.write_all(&compressed_bytes) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to write compressed data to destination file: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(compressed_file)
    }
}
