use std::process;

mod cmd;
mod input_processor;

fn main() {
    if let Err(err) = cmd::run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
