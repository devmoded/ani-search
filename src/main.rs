mod cli;
mod error;
mod output;
mod providers;
mod types;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    match cli::run().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {}\n{}", err, err.root_cause())
        }
    }
}
