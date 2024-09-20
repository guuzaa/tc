rust_i18n::i18n!("locales");

use std::process;

mod cmd;
mod counts;
mod input_processor;
mod locales;

fn main() {
    if cmd::run().is_err() {
        process::exit(1);
    }
}
