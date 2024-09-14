use std::process;

mod cmd;
mod input_processor;

fn main() {
    if cmd::run().is_err() {
        process::exit(1);
    }
}
