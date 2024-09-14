use std::process;

mod cmd;
mod input_processor;

fn main() {
    if let Err(_) = cmd::run() {
        process::exit(1);
    }
}
