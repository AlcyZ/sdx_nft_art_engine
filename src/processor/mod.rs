use std::path::Path;

use anyhow::Result;

use crate::config::app::AppConfiguration;
use crate::config::edition::EditionConfiguration;
use crate::hashing::simple_sha256;
use crate::layers_model::Layers;
use crate::logger::log_info;

pub fn create_images<L: AsRef<Path>, D: AsRef<Path>>(
    _layers: &Layers,
    _edition_config: &EditionConfiguration,
    _app_config: &AppConfiguration<L, D>,
) -> Result<()> {
    let _test_hash = simple_sha256("test".as_bytes());
    log_info("start processing images");

    Ok(())
}
