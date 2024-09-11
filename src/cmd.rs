use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader};

use crate::input_processor::{process_input, CountOptions};

#[derive(Parser)]
#[command(author, version, about = "A simple count program by Rust and Cursor")]
pub struct Cli {
    /// Show line count
    #[arg(short = 'l', long)]
    lines: bool,

    /// Show word count
    #[arg(short = 'w', long)]
    words: bool,

    /// Show character count
    #[arg(short = 'c', long)]
    chars: bool,

    /// Show token count
    #[arg(short = 't', long)]
    tokens: bool,

    /// Input files
    #[arg(name = "FILE")]
    files: Vec<String>,
}

impl Cli {
    fn parse_args() -> (Self, CountOptions) {
        let cli = Self::parse();

        let options = CountOptions {
            show_lines: cli.lines,
            show_words: cli.words,
            show_bytes: cli.chars,
            show_tokens: cli.tokens,
        };

        // If no options are specified, show all
        let options = if !options.show_lines
            && !options.show_words
            && !options.show_bytes
            && !options.show_tokens
        {
            CountOptions {
                show_lines: true,
                show_words: true,
                show_bytes: true,
                show_tokens: true,
            }
        } else {
            options
        };

        (cli, options)
    }
}

pub fn run() -> io::Result<()> {
    let (cli, options) = Cli::parse_args();

    let mut stdout = io::stdout();

    if cli.files.is_empty() {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin.lock());
        if let Err(err) = process_input(&mut reader, &mut stdout, &options) {
            eprintln!("Error processing stdin: {}", err);
        }
    } else {
        for filename in cli.files {
            match File::open(&filename) {
                Ok(mut file) => {
                    if let Err(err) = process_input(&mut file, &mut stdout, &options) {
                        eprintln!("Error processing file '{}': {}", filename, err);
                    }
                }
                Err(err) => {
                    eprintln!("Error opening file '{}': {}", filename, err);
                }
            }
        }
    }

    Ok(())
}
