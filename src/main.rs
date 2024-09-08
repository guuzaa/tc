use clap::{CommandFactory, Parser};
use std::fs::File;
use std::io::{self, BufReader};

mod input_processor;
use input_processor::{process_input, CountOptions};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A simple word count program by Rust and Cursor"
)]
struct Cli {
    /// Show line count
    #[arg(short = 'l', long)]
    lines: bool,

    /// Show word count
    #[arg(short = 'w', long)]
    words: bool,

    /// Show character count
    #[arg(short = 'c', long)]
    chars: bool,

    /// Input files
    #[arg(name = "FILE")]
    files: Vec<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // If no arguments are provided, print help and exit
    if cli.files.is_empty() && !cli.lines && !cli.words && !cli.chars {
        Cli::command().print_help().unwrap();
        return Ok(());
    }

    let options = CountOptions {
        show_lines: cli.lines,
        show_words: cli.words,
        show_bytes: cli.chars,
    };

    // If no options are specified, show all
    let options = if !options.show_lines && !options.show_words && !options.show_bytes {
        CountOptions {
            show_lines: true,
            show_words: true,
            show_bytes: true,
        }
    } else {
        options
    };

    let mut stdout = io::stdout();

    if cli.files.is_empty() {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        process_input(&mut handle, &mut stdout, &options)?;
    } else {
        for filename in cli.files {
            if filename == "-" {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                process_input(&mut handle, &mut stdout, &options)?;
            } else {
                let file = File::open(filename)?;
                let mut reader = BufReader::new(file);
                process_input(&mut reader, &mut stdout, &options)?;
            }
        }
    }

    Ok(())
}
