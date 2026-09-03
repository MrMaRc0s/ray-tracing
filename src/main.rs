use std::{
    io::{self, BufWriter},
    process,
};

use rt::{render_preview, CliCommand};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        eprintln!("try 'rt --help' for usage");
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    match CliCommand::from_env()? {
        CliCommand::Help => {
            print!("{}", rt::cli::HELP);
            Ok(())
        }
        CliCommand::Render(options) => {
            let canvas = render_preview(&options)?;
            let stdout = io::stdout();
            let mut output = BufWriter::new(stdout.lock());
            canvas
                .write_ppm(&mut output)
                .map_err(|error| format!("failed to write PPM image: {error}"))
        }
    }
}
