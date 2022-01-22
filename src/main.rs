#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod app;
mod config;
mod hashing;
mod layer;
mod logger;

fn main() -> Result<()> {
    app::run()?;

    Ok(())
}
