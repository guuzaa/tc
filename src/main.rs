use std::process;

mod cmd;
mod counts;
mod input_processor;

fn main() {
    if cmd::run().is_err() {
        process::exit(1);
    }
}
