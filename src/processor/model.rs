use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::imageops::overlay;
use image::{open, ImageBuffer, Rgba};

use crate::config::app::AppConfiguration;
use crate::hashing::simple_sha256;
use crate::layers_model::RngLayerFile;
use crate::logger::log_info;

#[derive(Debug)]
pub(super) struct ImageComposite {
    files: Vec<ImageCompositeFile>,
    dna: String,
}

impl ImageComposite {
    pub(super) fn from_rng_files(files: &Vec<RngLayerFile>) -> ImageComposite {
        let composite_files = files
            .iter()
            .map(|f| ImageCompositeFile::try_from_path(f.get_layer(), f.get_path()))
            .filter_map(|r| r.ok())
            .collect::<Vec<ImageCompositeFile>>();

        let dna_string = composite_files
            .iter()
            .map(|f| f.path.display().to_string())
            .collect::<Vec<String>>()
            .join("__");
        let dna = simple_sha256(dna_string.as_bytes());

        ImageComposite {
            files: composite_files,
            dna,
        }
    }

    pub(super) fn get_dna(&self) -> &str {
        &self.dna
    }

    pub(super) fn save<L: AsRef<Path>, D: AsRef<Path>>(
        &self,
        edition: u32,
        app_config: &AppConfiguration<L, D>,
    ) -> Result<()> {
        let context = "Save image composite";

        self.save_image(edition, app_config).context(context)?;

        Ok(())
    }

    fn save_image<L: AsRef<Path>, D: AsRef<Path>>(
        &self,
        edition: u32,
        app_config: &AppConfiguration<L, D>,
    ) -> Result<()> {
        let context = "Save overlayed images of composite";

        let image_paths = self
            .files
            .iter()
            .map(|f| f.path.as_path())
            .collect::<Vec<&Path>>();

        let final_image = overlay_images(1024, &image_paths).context(context)?;
        let destination = app_config.get_destination_dir().join("images");
        if !destination.is_dir() {
            create_dir_all(&destination).context(context)?;
        }
        let destination = destination.join(format!("{}.png", edition));
        final_image.save(&destination).context(context)?;

        log_info(format!(
            "Created unique image for edition: {}",
            destination.display()
        ));

        Ok(())
    }
}

#[derive(Debug)]
struct ImageCompositeFile {
    _file_name: String,
    _name: String,
    _layer: String,
    path: PathBuf,
}

impl ImageCompositeFile {
    fn try_from_path(layer: &str, path: &Path) -> Result<ImageCompositeFile> {
        let context = format!(
            "try to create image composite file for layer ({}) from path ({})",
            layer,
            path.display()
        );

        let file_name = try_convert_os_str_to_string(path.file_name().context(context.clone())?)
            .context(context.clone())?;
        let name = file_name
            .split('.')
            .next()
            .context(context.clone())?
            .to_string();

        Ok(ImageCompositeFile {
            _file_name: file_name,
            _name: name,
            _layer: layer.to_string(),
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
