use core::fmt;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
};

pub struct CursorConfig {
    pub cursor_size: u8,
    pub cursor_offset_x: u8,
    pub cursor_offset_y: u8,
    pub cursor_image: String,
}

pub fn parse_config_file(file: &PathBuf) -> Vec<CursorConfig> {	
    // Try opening config file.
    let config_file = match std::fs::File::open(file) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("Couldn't read config file. {:?} does not exist.", file)
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Couldn't read config file, lacking permissions.")
                }
                _ => {
                    eprintln!("Couldn't read config file:\n\n{}", e)
                }
            }

            exit(1);
        }
    };

    // Since we could open config file, let's start parsing.
    let mut configs: Vec<CursorConfig> = Vec::new();

    // Create a buffer with this file.
    let mut buffer = BufReader::new(config_file);

    if let Ok(buf) = &buffer.fill_buf() {
        // Check if it is empty.
        if buf == &[] {
            eprintln!("Empty config file");
            exit(1);
        }
    }

    let lines = buffer.lines();

    // Parse each line.
    for line in lines {
        /* 
         * Expect that we can get a line since we
         * already confirmed that buffer isn't
         * empty.
         */ 
        configs.push(match parse_config_line(line.expect("Expected a line")) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        });
    }

    configs
}

fn parse_config_line(line: String) -> Result<CursorConfig, ParseConfigLineError> {
    let chunks: Vec<&str> = line.splitn(4, ' ').collect();

    if !chunks.len() == 4 {
        return Err(ParseConfigLineError::WrongChunksCount(chunks.len()));
    }

    let config = CursorConfig {
        cursor_size: get_number_from_chunk(chunks[0]),
        cursor_offset_x: get_number_from_chunk(chunks[1]),
        cursor_offset_y: get_number_from_chunk(chunks[2]),
        cursor_image: chunks[3].to_owned(),
    };

    Ok(config)
}

/// Utility function to parse a value from the chunk of config's line gracefully.
fn get_number_from_chunk(chunk: &str) -> u8 {
    match chunk.parse::<u8>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid line argument: {}", e);
            exit(1);
        }
    }
}

/*
 * Errors
 */

#[derive(Debug, Clone)]
enum ParseConfigLineError {
    WrongChunksCount(usize),
}

struct WrongChunksCount(usize);
impl fmt::Display for WrongChunksCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid count of config line arguments, must be 4, got {}.",
            self
        )
    }
}

impl fmt::Display for ParseConfigLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing config's line: {}", self)
    }
}
