use std::process;

mod cmd;
mod input_processor;

#[tokio::main]
async fn main() {
    if let Err(err) = cmd::run().await {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
