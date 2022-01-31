use std::ffi::OsStr;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::imageops::overlay;
use image::{open, ImageBuffer, Rgba};

use crate::config::app::AppConfiguration;
use crate::config::edition::{EditionConfiguration, LayerConfiguration};
use crate::hashing::simple_sha256;
use crate::layers_model::{Layers, RngLayerFile};
use crate::logger::log_info;
use crate::processor::model::meta::Meta;

#[derive(Debug)]
pub(in super::super) struct Image {
    files: Vec<ImageFile>,
    dna: String,
}

impl Image {
    pub(in super::super) fn from_layers(
        layers: &Layers,
        layer_config: &LayerConfiguration,
    ) -> Image {
        let mut composite_files = vec![];
        layer_config.get_order().iter().for_each(|lo| {
            composite_files.append(&mut layers.get_rng_files(
                lo.get_name(),
                lo.get_pick_min(),
                lo.get_pick_max(),
            ))
        });

        Image::from_rng_files(&composite_files)
    }

    pub(in super::super) fn get_dna(&self) -> &str {
        &self.dna
    }

    pub(in super::super) fn save<L: AsRef<Path>, D: AsRef<Path>>(
        &self,
        edition: u32,
        app_config: &AppConfiguration<L, D>,
        edition_config: &EditionConfiguration,
    ) -> Result<()> {
        let context = "Save image composite";

        self.save_image(edition, app_config.get_destination_dir())
            .context(context)?;
        self.save_meta(edition, edition_config, app_config.get_destination_dir())
            .context(context)?;

        log_info(format!("Saved image and metadata #{}", edition,));

        Ok(())
    }
}

impl Image {
    fn from_rng_files(files: &[RngLayerFile]) -> Image {
        let composite_files = files
            .iter()
            .map(|f| ImageFile::try_from_path(f.get_layer(), f.get_path()))
            .filter_map(|r| r.ok())
            .collect::<Vec<ImageFile>>();

        let dna_string = composite_files
            .iter()
            .map(|f| f.path.display().to_string())
            .collect::<Vec<String>>()
            .join("__");
        let dna = simple_sha256(dna_string.as_bytes());

        Image {
            files: composite_files,
            dna,
        }
    }

    fn save_image<P: AsRef<Path>>(&self, edition: u32, destination_path: P) -> Result<()> {
        let context = format!(
            "Save edition ({}) image at ({})",
            edition,
            destination_path.as_ref().display()
        );

        let image_paths = self
            .files
            .iter()
            .map(|f| f.path.as_path())
            .collect::<Vec<&Path>>();

        let final_image = overlay_images(1024, &image_paths).context(context.clone())?;
        let destination = destination_path.as_ref().join("images");
        if !destination.is_dir() {
            create_dir_all(&destination).context(context.clone())?;
        }
        let destination = destination.join(format!("{}.png", edition));
        final_image.save(&destination).context(context)?;

        Ok(())
    }

    fn save_meta<P: AsRef<Path>>(
        &self,
        edition: u32,
        edition_config: &EditionConfiguration,
        destination_path: P,
    ) -> Result<()> {
        let context = format!(
            "Save edition ({}) meta at ({})",
            edition,
            destination_path.as_ref().display()
        );

        let meta = Meta::new(edition, edition_config, &self.files);
        let meta_string = serde_json::to_string_pretty(&meta).context(context.clone())?;

        let destination = destination_path.as_ref().join("meta");
        if !destination.is_dir() {
            create_dir_all(&destination).context(context.clone())?;
        }
        let destination = destination.join(format!("{}.json", edition));
        write(&destination, meta_string).context(context)?;

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct ImageFile {
    _file_name: String,
    name: String,
    layer: String,
    path: PathBuf,
}

impl ImageFile {
    pub(super) fn get_name(&self) -> &str {
        &self.name
    }
    pub(super) fn get_layer(&self) -> &str {
        &self.layer
    }
}

impl ImageFile {
    fn try_from_path(layer: &str, path: &Path) -> Result<ImageFile> {
        let context = format!(
            "try to create image composite file for layer ({}) from path ({})",
            layer,
            path.display()
        );

        let file_name = try_convert_os_str_to_string(path.file_name().context(context.clone())?)
            .context(context.clone())?;
        let name = file_name.split('.').next().context(context)?.to_string();

        Ok(ImageFile {
            _file_name: file_name,
            name,
            layer: layer.to_string(),
            path: path.to_path_buf(),
        })
    }
}

fn try_convert_os_str_to_string(str: &OsStr) -> Result<String> {
    let string = str
        .to_str()
        .context("try convert os str to string")?
        .to_string();

    Ok(string)
}

fn overlay_images<P: AsRef<Path>>(
    size: u32,
    image_paths: &[P],
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let context = "Overlay image layers";

    // transparent black background image as base
    let bg_cb = |_, _| Rgba([0u8, 0u8, 0u8, 0u8]);
    let mut base_img = ImageBuffer::from_fn(size, size, bg_cb);

    for image_path in image_paths {
        let dyn_img = open(image_path).context(context)?;
        let img = dyn_img.into_rgba8();

        overlay(&mut base_img, &img, 0, 0);
    }

    Ok(base_img)
}
