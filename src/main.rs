use std::process;

mod input_processor;
mod cmd;

fn main() {
    if let Err(err) = cmd::run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
