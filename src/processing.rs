use core::fmt;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
};

#[derive(Debug, PartialEq)]
/// This struct contains information about cursor's size, image and hotspot coordinates.
pub struct CursorInfo {
    pub size: u16,
    pub hotspot_x: u16,
    pub hotspot_y: u16,
    pub image: String,
}

pub fn parse_config_file(file: &PathBuf) -> Vec<CursorInfo> {
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
    let mut infos: Vec<CursorInfo> = Vec::new();

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
        infos.push(match CursorInfo::from(line.expect("Expected a line")) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        });
    }

    infos
}

impl CursorInfo {
    /// Parses cursor's info from a string of parameters
    /// ```
    /// # use wincursorgen::processing::{CursorInfo, ParseConfigLineError};
    /// let params = String::from("32 1 2 32-cursor-image.png");
    /// let info = CursorInfo::from(params)?;
    ///
    /// assert_eq!(
    ///     info,
    ///     CursorInfo {
    ///         size: 32u16,
    ///         hotspot_x: 1u16,
    ///         hotspot_y: 2u16,
    ///         image: String::from("32-cursor-image.png")
    ///     }
    /// );
    /// # Ok::<(), ParseConfigLineError>(())
    /// ```
    /// Notice that it won't fail if you supply more than 4 arguments, since we are using only first 4 anyway.
    /// ```
    /// # use wincursorgen::processing::{CursorInfo, ParseConfigLineError};
    /// let params = String::from("32 1 2 32-cursor-image.png AbraCadabra");
    /// let info = CursorInfo::from(params)?;
    ///
    /// assert_eq!(
    ///     info,
    ///     CursorInfo {
    ///         size: 32u16,
    ///         hotspot_x: 1u16,
    ///         hotspot_y: 2u16,
    ///         image: String::from("32-cursor-image.png")
    ///     }
    /// );
    /// # Ok::<(), ParseConfigLineError>(())
    /// ```
    /// This should fail even without the `assert_eq!`:
    /// ```should_panic
    /// # use wincursorgen::processing::{CursorInfo, ParseConfigLineError};
    /// let params = String::from("32 1 2");
    /// let info = CursorInfo::from(params)?;
    /// # Ok::<(), ParseConfigLineError>(())
    /// ```
    pub fn from(params: String) -> Result<CursorInfo, ParseConfigLineError> {
        // This will be 4 if cursor isn't animated and 5 if it is...
        let params: Vec<&str> = params.split(' ').collect();

        // but it can never be less than 4. That's illegal. >:C
        if params.len() < 4 {
            return Err(ParseConfigLineError::WrongChunksCount(params.len()));
        }

        let config = CursorInfo {
            size: parse_number(params[0]),
            hotspot_x: parse_number(params[1]),
            hotspot_y: parse_number(params[2]),
            image: params[3].to_owned(),
        };

        Ok(config)
    }
}

/// Utility function to parse a value from the chunk of config's line gracefully.
fn parse_number(part: &str) -> u16 {
    match part.parse::<u16>() {
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
pub enum ParseConfigLineError {
    WrongChunksCount(usize),
}
impl fmt::Display for ParseConfigLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing config's line: {}", self)
    }
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