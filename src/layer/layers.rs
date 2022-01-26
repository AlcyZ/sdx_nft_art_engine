use std::ffi::OsString;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::{GenericImageView, ImageBuffer, open, Rgba};
use image::imageops::{FilterType, overlay};
use rand::prelude::*;

use crate::config::{Configuration, LayerConfiguration};
use crate::hashing::simple_sha256;
use crate::layer::layer::Layer;
use crate::layer::layer_composite::FinalLayerComposite;
use crate::logger::{log_info, log_measure, log_warn};

#[derive(Debug)]
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    pub fn from_configuration(configuration: &Configuration) -> Result<Layers> {
        let context = "create layers from configuration";

        let mut layers = vec![];
        for entry in read_dir(configuration.get_layers_dir())
            .context(context)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
        {
            layers.push(Layer::try_from_dir_entry(entry).context(context)?);
        }

        Ok(Layers { layers })
    }

    pub fn create_images(
        &self,
        layer_config: &LayerConfiguration,
        config: &Configuration,
    ) -> Result<()> {
        let context = format!(
            "Create images from layer configuration {})",
            &layer_config.get_dna()[..6]
        );

        let mut existing_dna: Vec<String> = vec![];
        let edition_size = layer_config.get_size();
        let mut editions = 0;
        let mut retries = 0;
        let max_tries = config.get_max_tries();

        let log = log_measure(context.to_string());
        while editions < edition_size && retries < max_tries {
            let final_composite = self.get_final_layer_composite(layer_config);
            let composite_dna = final_composite.get_dna();
            if existing_dna.contains(&composite_dna.to_string()) {
                retries += 1;

                Layers::check_log_existing_dna(retries, composite_dna);
            } else {
                final_composite
                    .save(editions, layer_config, config)
                    .context(context.to_string())?;
                existing_dna.push(composite_dna.to_string());
                editions += 1;
            }
        }
        log.finish();

        Ok(())
    }
}

impl Layers {
    fn get_layer<N: AsRef<str>>(&self, name: N) -> Option<&Layer> {
        self.layers.iter().find(|l| l.get_name() == name.as_ref())
    }

    fn get_final_layer_composite(
        &self,
        layer_configuration: &LayerConfiguration,
    ) -> FinalLayerComposite {
        let mut files = vec![];

        for name in layer_configuration.get_order() {
            if let Some(layer) = self.get_layer(name.get_name()) {
                let mut layer_files = layer.get_rng_files(name.get_pick_min(), name.get_pick_max());

                files.append(&mut layer_files)
            }
        }

        FinalLayerComposite::new(files)
    }
}

impl Layers {
    fn check_log_existing_dna(retries: u32, composite_dna: &str) {
        if retries < 1000 {
            Layers::log_existing_dna(retries, composite_dna);
            return;
        }

        if retries < 3000 && retries % 100 == 0 {
            Layers::log_existing_dna(retries, composite_dna);
            return;
        }

        if retries < 5000 && retries % 250 == 0 {
            Layers::log_existing_dna(retries, composite_dna);
            return;
        }

        if retries % 500 == 0 {
            Layers::log_existing_dna(retries, composite_dna);
        }
    }

    fn log_existing_dna(retries: u32, composite_dna: &str) {
        log_warn(format!(
            "DNA already exists! ({})\t|\t Retry! ({})",
            &composite_dna[..6],
            retries
        ));
    }
}