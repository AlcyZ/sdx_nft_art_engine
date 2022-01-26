use std::ffi::OsString;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::{GenericImageView, ImageBuffer, open, Rgba};
use image::imageops::{FilterType, overlay};
use rand::prelude::*;

use crate::config::{Configuration, LayerConfiguration};
use crate::hashing::simple_sha256;
use crate::layer::layer::LayerFile;
use crate::logger::{log_info, log_measure, log_warn};

pub(super) struct FinalLayerComposite {
    dna: String,
    files: Vec<LayerFile>,
}

impl FinalLayerComposite {
    pub(super) fn new(files: Vec<LayerFile>) -> FinalLayerComposite {
        let dna_string = FinalLayerComposite::generate_name_from_filenames(&files);
        let dna = simple_sha256(dna_string.as_bytes());

        FinalLayerComposite { dna, files }
    }

    pub(super)  fn get_dna(&self) -> &str {
        &self.dna
    }

    fn generate_name_from_filenames(files: &[LayerFile]) -> String {
        files
            .iter()
            .map(|f| format!("{}:{}", f.get_id(), f.get_name()))
            .collect::<Vec<String>>()
            .join("-")
    }

    pub(super)  fn save(
        &self,
        index: u32,
        layer_config: &LayerConfiguration,
        config: &Configuration,
    ) -> Result<()> {
        let context = "save final layer composite";

        let image_paths = self
            .files
            .iter()
            .map(|f| f.get_path())
            .collect::<Vec<&Path>>();

        let final_img = overlay_images(
            config.get_image_size(),
            config.is_resize_enabled(),
            &image_paths,
        )
            .context(context)?;

        let config_dna = layer_config.get_dna();
        let layer_dna = self.get_dna();

        let file_name = format!(
            "{}_{}#{}.png",
            &config_dna[0..6],
            index + 1,
            &layer_dna[0..6],
        );
        let destination = config.get_destination_dir().join(file_name);

        log_info(format!("Save composed image at: {}", destination.display()));
        final_img.save(&destination).context(format!(
            "{} at {}",
            context,
            destination.display()
        ))?;

        Ok(())
    }
}

fn overlay_images<P: AsRef<Path>>(
    size: u32,
    resize: bool,
    image_paths: &[P],
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let context = "Overlay image layers";

    // transparent white background image as base
    let white_bg_cb = |_, _| Rgba([255u8, 255u8, 255u8, 0u8]);
    let mut base_img = ImageBuffer::from_fn(size, size, white_bg_cb);

    for image_path in image_paths {
        let mut dyn_img = open(image_path).context(context)?;

        // only resize if dimension mismatch
        if resize && dyn_img.dimensions().0 != size {
            dyn_img = dyn_img.resize(size, size, FilterType::Gaussian);
        }
        let img = dyn_img.into_rgba8();

        overlay(&mut base_img, &img, 0, 0);
    }

    Ok(base_img)
}