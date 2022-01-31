use anyhow::{Context, Result};

use crate::config::app::AppConfiguration;
use crate::config::edition::EditionConfiguration;
use crate::layers_model::Layers;
use crate::logger::log_measure;
use crate::processor::create_images;

pub fn run() -> Result<()> {
    let edition_config = EditionConfiguration::try_from_path("./config/layer_configuration.json")?;
    let app_config = AppConfiguration::new("./layers", "./build");
    let layers = Layers::from_config(&app_config);

    let log = log_measure("create images");
    create_images(&layers, &edition_config, &app_config).context("asd")?;
    log.finish();

    Ok(())
}
