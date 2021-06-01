use std::{error::Error, fs::File, io::BufWriter, path::PathBuf, process::exit};
use wincursorgen::{config::parse_config, cursor::generate_cur};
use wincursorgen::cursor::generate_ani;

fn main() -> Result<(), Box<dyn Error>> {
    let args = wincursorgen::cli::Arguments::parse()?;

    let parsed_cursors = match parse_config(args.config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    for ci in parsed_cursors {
        let (size, ci) = ci;

        match ci {
            wincursorgen::config::ConfigResult::Cursors(frames) => {
                let cursor = generate_ani(size, frames, args.prefix.clone());

                let writer = match File::create(args.output.with_extension("ani")) {
                    Ok(file) => BufWriter::new(file),
                    Err(error) => {
                        match error.kind() {
                            std::io::ErrorKind::PermissionDenied => {
                                eprintln!("Lacking permission to open a writer to output.")
                            }
                            _ => eprintln!(
                                "Something wild happened while trying to open a writer to output, please report: {}", error
                            ),
                        };
                        exit(1);
                    }
                };

                if let Err(error) = cursor.encode(writer) {
                    match error {
                        riff_ani::Error::Io(error) => {
                            match error.kind() {
                               
                            std::io::ErrorKind::PermissionDenied => {
                                eprintln!("Lacking permission to open a writer to output.")
                            }
                            _ => eprintln!(
                                "Something wild happened while trying to open a writer to output, please report: {}", error
                            ),
                            }
                        }
                    }
                    exit(1);
                }
            }
            wincursorgen::config::ConfigResult::Cursor(cursor) => {
                let cursor = generate_cur(
                    cursor.hotspot,
                    PathBuf::from(args.prefix.join(cursor.image)),
                );

                let writer = match File::create(args.output.with_extension("cur")) {
                    Ok(file) => BufWriter::new(file),
                    Err(error) => {
                        match error.kind() {
                            std::io::ErrorKind::PermissionDenied => {
                                eprintln!("Lacking permission to open a writer to output.")
                            }
                            _ => eprintln!(
                                "Something wild happened while trying to open a writer to output, please report: {}", error
                            ),
                        };
                        exit(1);
                    }
                };

                if let Err(error) = cursor.write(writer) {
                    match error.kind() {
                            std::io::ErrorKind::PermissionDenied => {
                                eprintln!("Lacking permission to open a writer to output.")
                            }
                            _ => eprintln!(
                                "Something wild happened while trying to open a writer to output, please report: {}", error
                            ),
                        };
                    exit(1);
                }
            }
        };
    }

    Ok(())
}
