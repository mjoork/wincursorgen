use std::{path::PathBuf, process::exit};

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Generate a windows cursor file from a series of PNG images
pub struct Arguments {
    /// image files directory
    #[argh(option, short = 'p')]
    pub prefix: PathBuf,
    /// config file
    #[argh(positional)]
    pub config: PathBuf,
    /// output file
    #[argh(positional)]
    pub output: PathBuf,
}

use path_clean::PathClean;
impl Arguments {
    pub fn parse() -> Self {
        let mut args: Arguments = argh::from_env();

        // Clean paths.
        args.prefix = args.prefix.clean();
        args.config = args.config.clean();
        args.output = args.output.clean();

        // Check that resources is a directory.
        if !args.prefix.is_dir() {
            eprintln!("Provided resources path is not a directory. Must be.");
            exit(1);
        }

        args
    }
}
