use std::{path::PathBuf, process::exit};

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Generate a windows cursor file from a series of PNG images
pub struct Arguments {
    /// image files directory
    #[argh(option, short = 'r')]
    pub resources: PathBuf,
    /// config file
    #[argh(option, short = 'c')]
    pub config: PathBuf,
    /// output file
    #[argh(option, short = 'o')]
    pub output: PathBuf,
}

use path_clean::PathClean;
impl Arguments {
    pub fn parse() -> Self {
        let mut args: Arguments = argh::from_env();

        // Clean paths.
        args.resources = args.resources.clean();
        args.config = args.config.clean();
        args.output = args.output.clean();

        // Check that resources is a directory.
        if !args.resources.is_dir() {
            eprintln!("Provided resources path is not a directory. Must be.");
            exit(1);
        }

        args
    }
}
