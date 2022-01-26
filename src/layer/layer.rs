use std::ffi::OsString;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::{GenericImageView, ImageBuffer, open, Rgba};
use image::imageops::{FilterType, overlay};
use rand::prelude::*;

use crate::config::{Configuration, LayerConfiguration};
use crate::hashing::simple_sha256;
use crate::logger::{log_info, log_measure, log_warn};

#[derive(Debug, Clone)]
pub struct Layer {
    name: String,
    files: LayerFiles,
}

impl Layer {
    pub(super) fn try_from_dir_entry(entry: DirEntry) -> Result<Layer> {
        let context = format!(
            "try to create layer file from directory entry: {}",
            entry.path().display()
        );

        let name = try_convert_os_string_to_string(entry.file_name()).context(context.clone())?;
        let mut files = vec![];
        for (id, layer_entry) in read_dir(entry.path())
            .context(context.clone())?
            .filter_map(|e| e.ok())
            .enumerate()
        {
            files.push(LayerFile::try_from_dir_entry(id, layer_entry).context(context.clone())?);
        }
        if files.is_empty() {
            bail!(
                "Couldn't find any layer files in {}",
                entry.path().display()
            );
        }
        let files = LayerFiles::new(files);

        Ok(Layer { name, files })
    }

    pub(super) fn get_rng_files(&self, min: u32, max: u32) -> Vec<LayerFile> {
        assert!(min <= max, "Min must be lower or equal to max!");

        let to = if min == max {
            min
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..max)
        };
        let mut files = vec![];

        for _ in 0..to {
            files.push(self.get_rng_file().clone());
        }

        files
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
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
pub(super) struct LayerFile {
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


    pub(super) fn get_id(&self) -> usize {
        self.id
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
    }

    pub(super) fn get_path(&self) -> &Path {
        &self.path
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
