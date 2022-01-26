#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod app;
mod config;
mod hashing;
mod logger;
mod processor;

fn main() -> Result<()> {
    app::run()?;

    Ok(())
}
