use std::process;

rust_i18n::i18n!("locales");

mod cmd;
mod counts;
mod input_processor;
mod locales;

fn main() {
    if cmd::run().is_err() {
        process::exit(1);
    }
}
