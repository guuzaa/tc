use clap::{CommandFactory, Parser};
use tokio::fs::File;
use tokio::io::{self, BufReader};

use crate::input_processor::{process_input, CountOptions};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A simple token count program by Rust and Cursor"
)]
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

    /// Input files
    #[arg(name = "FILE")]
    files: Vec<String>,
}

impl Cli {
    fn parse_args() -> (Self, CountOptions) {
        let cli = Self::parse();

        // If no arguments are provided, print help and exit
        if cli.files.is_empty() && !cli.lines && !cli.words && !cli.chars {
            Self::command().print_help().unwrap();
            std::process::exit(0);
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

        (cli, options)
    }
}

pub async fn run() -> io::Result<()> {
    let (cli, options) = Cli::parse_args();

    let mut stdout = tokio::io::stdout();

    if cli.files.is_empty() {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        if let Err(err) = process_input(&mut reader, &mut stdout, &options).await {
            eprintln!("Error processing stdin: {}", err);
        }
    } else {
        for filename in cli.files {
            match File::open(&filename).await {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    if let Err(err) = process_input(&mut reader, &mut stdout, &options).await {
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
