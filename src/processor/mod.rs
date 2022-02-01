use std::fmt::Debug;
use std::fs::remove_dir_all;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::app::AppConfiguration;
use crate::config::edition::EditionConfiguration;
use crate::layers_model::Layers;
use crate::logger::{log_info, log_warn};
use crate::processor::model::image::Image;

mod model;

pub fn create_images<L: AsRef<Path> + Debug, D: AsRef<Path> + Debug>(
    layers: &Layers,
    edition_config: &EditionConfiguration,
    app_config: &AppConfiguration<L, D>,
) -> Result<()> {
    let mut edition_size = 0;
    let mut edition_items = 0;
    let mut retries = 0;
    let mut existing_dna: Vec<String> = vec![];
    let max_tries = app_config.get_max_tries();

    if app_config.is_cleanup_enabled() {
        log_info("Cleanup destination directory");
        remove_dir_all(app_config.get_destination_dir())
            .context("cleanup destination directory before image processing")?;
    }

    for layer_config in edition_config.get_layers() {
        edition_size += layer_config.get_size();

        while edition_items < edition_size && retries < max_tries {
            let composite = Image::from_layers(layers, layer_config);
            let composite_dna = composite.get_dna().to_string();

            if existing_dna.contains(&composite_dna) {
                retries += 1;
                check_log_existing_dna(retries, &composite_dna);
            } else {
                composite
                    .save(edition_items + 1, app_config, edition_config)
                    .context("save composite while image processing")?;

                existing_dna.push(composite_dna);
                edition_items += 1;
            }
        }
    }

    Ok(())
}

fn check_log_existing_dna(retries: u32, composite_dna: &str) {
    if retries < 1000 {
        log_existing_dna(retries, composite_dna);
        return;
    }

    if retries < 3000 && retries % 100 == 0 {
        log_existing_dna(retries, composite_dna);
        return;
    }

    if retries < 5000 && retries % 250 == 0 {
        log_existing_dna(retries, composite_dna);
        return;
    }

    if retries % 500 == 0 {
        log_existing_dna(retries, composite_dna);
    }
}

fn log_existing_dna(retries: u32, composite_dna: &str) {
    log_warn(format!(
        "DNA already exists! ({})\t|\t Retry! ({})",
        &composite_dna[..6],
        retries
    ));
}
