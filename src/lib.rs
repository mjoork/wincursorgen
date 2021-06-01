/// Milliseconds one jiffy is equal to.
const JIFFY: f32 = 16.6666;

/// This module contains application-specific errors.
/// The pattern for this module is simple:
/// ```ignore
/// pub mod errors {
/// 	#[derive(Debug)]
/// 	pub enum ErrorName {
/// 		ErrorVariant,
/// 		OtherErrorVariant
/// 	}
///
/// 	impl std::fmt::Display for ErrorName { /* ... */ }
///
///		// ...other Error enum-s + Display impl.
///
///		impl Error for ErrorName {}
/// 	// ...other Error impl-s
/// }
/// ```
pub mod errors {
    use std::{
        error::Error,
        fmt::{Display, Formatter, Result},
    };

    #[derive(Debug)]
    /// The error that might occur during checking of arguments. I still call it
    /// a parse error because it occurs before program even starts processing.
    pub enum FileError {
        PrefixIsNotDir,
        ConfigDoesNotExist,
        ConfigIsNotAFile,
    }

    impl Display for FileError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            // The actual error message.
            match self {
                FileError::PrefixIsNotDir => write!(f, "Prefix is not a directory."),
                FileError::ConfigDoesNotExist => write!(f, "Config does not exist."),
                FileError::ConfigIsNotAFile => write!(f, "Config isn't a file."),
            }
        }
    }

    #[derive(Debug)]
    pub enum ConfigLineParsingError {
        NoCursorSize,
        NoHotspotX,
        NoHotspotY,
        NoCursorImage,
    }

    impl Display for ConfigLineParsingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ConfigLineParsingError::NoCursorSize => {
                    write!(f, "Cursor's size is not defined in config file.")
                }
                ConfigLineParsingError::NoHotspotX => {
                    write!(f, "Cursor's hotspot x-coord is not defined in config file.")
                }
                ConfigLineParsingError::NoHotspotY => {
                    write!(f, "Cursor's hotspot y-coord is not defined in config file.")
                }
                ConfigLineParsingError::NoCursorImage => {
                    write!(f, "Cursor's image is not defined in config file.")
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum ConfigParsingError {
        OnLine(u8, ConfigLineParsingError),
        ConfigEmpty,
    }

    impl Display for ConfigParsingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ConfigParsingError::OnLine(line_number, line_error) => write!(
                    f,
                    "Failed parsing config file - line: {} - error: {}",
                    line_number, line_error
                ),
                ConfigParsingError::ConfigEmpty => write!(f, "Config is empty."),
            }
        }
    }

    // Let it generate default implementations.
    impl Error for FileError {}
    impl Error for ConfigParsingError {}
    impl Error for ConfigLineParsingError {}
}

/// This module is just for arguments parsing.
pub mod cli {
    use crate::errors::FileError;
    use argh::FromArgs;
    use path_clean::PathClean;
    use std::path::PathBuf;

    #[derive(FromArgs, Debug)]
    /// Generate a Windows cursor file from a series of PNG images
    pub struct Arguments {
        /// image files directory
        #[argh(option, short = 'p')]
        pub prefix: PathBuf,
        /// config file
        #[argh(positional)]
        pub config: PathBuf,
        /// output file; DO NOT add an extension to the filename; just use it as you would xcursorgen
        #[argh(positional)]
        pub output: PathBuf,
    }

    impl Arguments {
        /// This calls `argh::from_env()` and then does some prerun tests to
        /// ensure that basic conditions are met.
        pub fn parse() -> Result<Self, FileError> {
            let mut args: Arguments = argh::from_env();

            // Clean paths.
            args.prefix = args.prefix.clean();
            args.config = args.config.clean();
            args.output = args.output.clean();

            // Check that `arg.prefix` is a directory.
            if !args.prefix.is_dir() {
                return Err(FileError::PrefixIsNotDir);
            }

            // Check that `arg.config` exists and is a file.
            if !args.config.exists() {
                return Err(FileError::ConfigDoesNotExist);
            } else if !args.config.is_file() {
                return Err(FileError::ConfigIsNotAFile);
            }

            Ok(args)
        }
    }
}

pub mod config {
    use crate::errors::{ConfigLineParsingError, ConfigParsingError};
    use crate::JIFFY;
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
        process::exit,
    };

    #[derive(Debug)]
    pub struct CursorInformation {
        pub is_frame: bool,
        pub size: u16,
        pub hotspot: (u16, u16),
        pub image: String,
        pub delay: u16,
    }

    pub enum ConfigResult {
        Cursors(Vec<CursorInformation>),
        Cursor(CursorInformation),
    }

    /// Parses a config file.
    pub fn parse_config(config: PathBuf) -> Result<HashMap<u16, ConfigResult>, ConfigParsingError> {
        let config = File::open(config);

        let mut result: HashMap<u16, ConfigResult> = HashMap::new();

        match config {
            Ok(config) => {
                // Init reader buffer and check that file isn't empty.
                let mut reader = BufReader::new(config);
                if reader.fill_buf().unwrap_or(&[]) == &[] {
                    return Err(ConfigParsingError::ConfigEmpty);
                }

                // Read every line and parse cursor's information from them.
                let mut line_number = 1; // Count lines along the way.
                for line in reader.lines() {
                    let line = line.expect("Expected a line, found `None`.");

                    match CursorInformation::parse(line) {
                        // NOTE: This does in fact overwrite if there is for
                        // example first an animation is stored and then we
                        // store a static cursor of the same size, but this is
                        // just how it works, even xcursorgen doesn't do
                        // anything about it. It assumes that in one config
                        // there's either an animation or a static cursor, even
                        // if of different sizes.
                        Ok(ci) => {
                            // If cursor is a part of animation and there's no
                            // key for its' size, create a new vector for it and
                            // push it there.
                            if !result.contains_key(&ci.size) && ci.is_frame {
                                result.insert(ci.size, ConfigResult::Cursors(Vec::new())); // We just inserted a new vector there, so we
                                                                                           // unwrap, it should not cause an error.
                                if let ConfigResult::Cursors(v) = result.get_mut(&ci.size).unwrap()
                                {
                                    v.push(ci);
                                    continue;
                                }
                            }
                            // If cursor is a part of animated and there is a
                            // key for its' size, try to get a mutable reference
                            // to vector of frames in that key and push it into
                            // the vector.
                            else if result.contains_key(&ci.size) && ci.is_frame {
                                if let ConfigResult::Cursors(v) = result.get_mut(&ci.size).unwrap()
                                {
                                    v.push(ci);
                                    continue;
                                }
                            }

                            // If it isn't a frame, just put this cursor in value.
                            if !ci.is_frame {
                                result.insert(ci.size, ConfigResult::Cursor(ci));
                            }
                        }

                        // Tell the user on which line the error occured to
                        // spare them some time.
                        Err(error) => return Err(ConfigParsingError::OnLine(line_number, error)),
                    }

                    line_number += 1;
                }
            }

            // Config *must* exist, because we checked for it before executing
            // program. If it doesn't, then something really wild happened.
            Err(error) => {
                match error.kind() {
                            std::io::ErrorKind::NotFound => eprintln!("Cannot find config even though it existed before."),
                            std::io::ErrorKind::PermissionDenied => eprintln!("Lacking permissions to open config even though had before."),
                            _ => eprintln!("Some wild error occured while trying to read config, report this please: {}", error)
                        }
                exit(1);
            }
        };

        Ok(result)
    }

    /// Original implementation in xcursorgen uses `sscanf()` to parse config lines.
    /// This macro is a rough recreation of it.
    ///
    /// It conveniently returns every param wrapped in an `Option` so handling
    /// errors is a breeze.
    ///
    /// Credits: <https://stackoverflow.com/a/31048103/13709462>
    macro_rules! scan {
	    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
	        let mut iter = $string.split($sep);
	        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
	    }}
	}

    impl CursorInformation {
        /// Parses single config line and returns a `CursorInformation` variant, which
        /// is either a static cursor or a cursor's frame. The only difference between
        /// them is that the frame also has delay already converted to jiffies.
        fn parse(line: String) -> Result<CursorInformation, ConfigLineParsingError> {
            let (size, xhot, yhot, image, ms_delay) =
                scan!(line, char::is_whitespace, u16, u16, u16, String, u16);

            // Check that all of them exist, because all of them are required.
            let size = match size {
                Some(size) => size,
                None => return Err(ConfigLineParsingError::NoCursorSize),
            };

            let xhot = match xhot {
                Some(xhot) => xhot,
                None => return Err(ConfigLineParsingError::NoHotspotX),
            };

            let yhot = match yhot {
                Some(yhot) => yhot,
                None => return Err(ConfigLineParsingError::NoHotspotY),
            };

            let image = match image {
                Some(image) => image,
                None => return Err(ConfigLineParsingError::NoCursorImage),
            };

            // Return static if delay parameter is none.
            if ms_delay.is_none() {
                return Ok(CursorInformation {
                    is_frame: false,
                    size,
                    hotspot: (xhot, yhot),
                    image,
                    delay: 0,
                });
            }

            // Else return Cursor's frame.
            Ok(CursorInformation {
                is_frame: true,
                size,
                hotspot: (xhot, yhot),
                image,
                // We expect that there is a ms_delay parameter since we checked
                // whether it `is_none()` before.
                delay: convert_to_jiffies(ms_delay.expect("Expected a delay parameter")),
            })
        }
    }

    /// Converts milliseconds to jiffies, duh.
    fn convert_to_jiffies(ms: u16) -> u16 {
        (ms as f32 / JIFFY) as u16
    }
}

pub mod cursor {
    use std::{
        fs::File,
        io::{BufReader, ErrorKind},
        path::PathBuf,
        process::exit,
    };

    use riff_ani::{
        ico::{IconDir, IconDirEntry, IconImage, ResourceType},
        Ani, AniHeader,
    };

    use crate::config::CursorInformation;

    pub fn generate_cur(hotspot: (u16, u16), path: PathBuf) -> IconDir {
        let image = match File::open(path) {
            Ok(file) => BufReader::new(file),
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => eprintln!("Could not read image, not found."),
                    ErrorKind::PermissionDenied => {
                        eprintln!("Could not read image, lacking permissions")
                    }
                    _ => eprintln!(
                        "Some wild error occured while reading image, report please: {}",
                        error
                    ),
                }

                exit(1);
            }
        };

        let mut image = IconImage::read_png(image).unwrap();
        image.set_cursor_hotspot(Some(hotspot));
        let entry = IconDirEntry::encode(&image).unwrap();

        let mut cursor = IconDir::new(ResourceType::Cursor);
        cursor.add_entry(entry);

        cursor
    }

    pub fn generate_ani(size: u16, animation: Vec<CursorInformation>, prefix: PathBuf) -> Ani {
        let frame_rate = animation.first().unwrap().delay.clone() as u32;

        let frames = animation
            .iter()
            .map(|ci| generate_cur(ci.hotspot, PathBuf::from(prefix.join(&ci.image))))
            .collect();

        Ani {
            header: AniHeader {
                num_frames: animation.len() as u32,
                num_steps: animation.len() as u32,
                width: size as u32,
                height: size as u32,
                frame_rate,
            },
            frames,
        }
    }
}
