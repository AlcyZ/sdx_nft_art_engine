#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod app;
mod config;
mod hashing;
mod layers_model;
mod logger;
mod processor;

#[tokio::main]
async fn main() -> Result<()> {
    app::run()
}
