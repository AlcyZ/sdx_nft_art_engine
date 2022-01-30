#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod app;
mod config;
mod hashing;
mod logger;

#[tokio::main]
async fn main() -> Result<()> {
    app::run()
}
