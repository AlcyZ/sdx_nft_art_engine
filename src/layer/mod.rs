use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::imageops::{overlay, FilterType};
use image::{open, GenericImageView, ImageBuffer, Rgba};
use rand::prelude::*;

use crate::config::{Configuration, LayerConfiguration};
use crate::hashing::simple_sha256;
use crate::logger::{log_info, log_measure, log_warn};

#[derive(Debug)]
pub struct Layers {
    layers: Vec<Layer>,
}

struct _ImageMetaData {
    name: String,
    description: String,
    image: String,
    dna: String,
    edition: String,
    date: u64,
    attributes: Vec<_ImageMetaDataAttributes>,
}

impl _ImageMetaData {
    // fn new(name: String,
    //        description: String,
    //        image: String,
    //        dna: String,
    //        edition: String,
    //        date: u64,
    //        attributes: Vec<ImageMetaDataAttributes>) -> ImageMetaData {
    //     ImageMetaData {
    //         name,
    //         description,
    //         image,
    //         dna,
    //         edition,
    //         date,
    //         attributes,
    //     }
    // }

    fn _from_layer_composite() {
        todo!("implement it layer on");
    }
}

struct _ImageMetaDataAttributes {
    trait_type: String,
    value: String,
}

impl Layers {
    pub fn from_configuration(configuration: &Configuration) -> Result<Layers> {
        let context = "create layers from configuration";

        let mut layers = vec![];
        for entry in read_dir(configuration.get_layers_dir())
            .context(context)?
            .filter_map(|e| e.ok())
        {
            layers.push(Layer::try_from_dir_entry(entry).context(context)?);
        }

        Ok(Layers { layers })
    }

    pub fn create_images<T: AsRef<str>>(
        &self,
        layer_config: &LayerConfiguration<T>,
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

                check_log_existing_dna(retries, composite_dna);
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

impl Layers {
    fn get_layer<N: AsRef<str>>(&self, name: N) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name == name.as_ref())
    }

    fn get_final_layer_composite<T: AsRef<str>>(
        &self,
        layer_configuration: &LayerConfiguration<T>,
    ) -> FinalLayerComposite {
        let mut files = vec![];

        for name in layer_configuration.get_order() {
            if let Some(layer) = self.get_layer(name) {
                files.push(layer.get_rng_file().clone())
            }
        }

        FinalLayerComposite::new(files)
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    name: String,
    files: LayerFiles,
}

impl Layer {
    fn try_from_dir_entry(entry: DirEntry) -> Result<Layer> {
        let context = "try to create layer from directory entry";

        let name = try_convert_os_string_to_string(entry.file_name()).context(context)?;
        let mut files = vec![];
        for (id, layer_entry) in read_dir(entry.path())
            .context(context)?
            .filter_map(|e| e.ok())
            .enumerate()
        {
            files.push(LayerFile::try_from_dir_entry(id, layer_entry).context(context)?);
        }
        if files.is_empty() {
            bail!("Couldn't find any layer files.");
        }
        let files = LayerFiles::new(files);

        Ok(Layer { name, files })
    }

    fn get_rng_file(&self) -> &LayerFile {
        self.files.get_rng_file()
    }
}

#[derive(Debug, Clone)]
struct LayerFiles {
    layer_files: Vec<LayerFile>,
}

impl LayerFiles {
    fn new(layer_files: Vec<LayerFile>) -> LayerFiles {
        LayerFiles { layer_files }
    }

    fn get_rng_file(&self) -> &LayerFile {
        self.layer_files.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[derive(Debug, Clone)]
struct LayerFile {
    id: usize,
    name: String,
    _weight: u32,
    path: PathBuf,
}

impl LayerFile {
    fn try_from_dir_entry(id: usize, entry: DirEntry) -> Result<LayerFile> {
        let context = format!(
            "try to create layer file from directory entry: {}",
            entry.path().display()
        );

        let file_name = try_convert_os_string_to_string(entry.file_name()).context(context)?;
        let filename_parts = file_name.split('#').collect::<Vec<&str>>();
        let weight = if filename_parts.len() == 2 {
            let end_parts = filename_parts
                .last()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>();
            if end_parts.len() == 2 {
                end_parts.first().unwrap().parse::<u32>().unwrap_or(1)
            } else {
                1
            }
        } else {
            1
        };

        Ok(LayerFile {
            id,
            name: file_name.to_string(),
            _weight: weight,
            path: entry.path(),
        })
    }
}

struct FinalLayerComposite {
    dna: String,
    files: Vec<LayerFile>,
}

impl FinalLayerComposite {
    fn new(files: Vec<LayerFile>) -> FinalLayerComposite {
        let dna_string = FinalLayerComposite::generate_name_from_filenames(&files);
        let dna = simple_sha256(dna_string.as_bytes());

        FinalLayerComposite { dna, files }
    }

    fn get_dna(&self) -> &str {
        &self.dna
    }

    fn _get_name(&self) -> String {
        FinalLayerComposite::generate_name_from_filenames(&self.files)
    }

    fn generate_name_from_filenames(files: &[LayerFile]) -> String {
        files
            .iter()
            .map(|f| format!("{}:{}", f.id, f.name))
            .collect::<Vec<String>>()
            .join("-")
    }

    fn save<T: AsRef<str>>(
        &self,
        index: u32,
        layer_config: &LayerConfiguration<T>,
        config: &Configuration,
    ) -> Result<()> {
        let context = "save final layer composite";

        let image_paths = self
            .files
            .iter()
            .map(|f| f.path.as_path())
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

fn try_convert_os_string_to_string(os_string: OsString) -> Result<String> {
    Ok(os_string
        .to_str()
        .ok_or_else(|| {
            anyhow!("Failed to convert OsString into a Rust String")
                .context("Try to convert os string into rust string")
        })?
        .to_string())
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
