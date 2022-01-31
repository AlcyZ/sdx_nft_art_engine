use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rand::prelude::*;

use crate::config::app::AppConfiguration;
use crate::logger::log_warn;

#[derive(Debug)]
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    pub fn get_rng_files(&self, name: &str, min: u32, max: u32) -> Vec<RngLayerFile> {
        match self.find_layer(name) {
            Some(layer) => {
                let mut rng = rand::thread_rng();
                let amount = if min == max {
                    min
                } else {
                    rng.gen_range(min..=max)
                };

                sort_utility(
                    layer
                        .files
                        .choose_multiple(&mut rand::thread_rng(), amount as usize)
                        .map(|p| p.to_path_buf())
                        .map(|p| RngLayerFile {
                            layer: name.to_string(),
                            path: p,
                        })
                        .collect(),
                )
            }
            None => {
                log_warn(format!("Couldn't find layer with name: {}", name));

                vec![]
            }
        }
    }

    fn find_layer(&self, name: &str) -> Option<&Layer> {
        self.layers.iter().find(|l| l._name == name)
    }
}

impl Layers {
    pub fn from_config<L: AsRef<Path>, D: AsRef<Path>>(
        app_config: &AppConfiguration<L, D>,
    ) -> Layers {
        Layers::from_path(app_config.get_layers_dir())
    }

    fn from_path<P: AsRef<Path>>(path: P) -> Layers {
        let layers = match read_dir(path) {
            Ok(dir) => {
                let mut layers = vec![];
                for entry in dir.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()) {
                    match Layer::try_from_dir_entry(&entry) {
                        Ok(layer) => layers.push(layer),
                        Err(err) => {
                            let message = format!(
                                "Couldn't create layer from dir entry: {}\n{}",
                                entry.path().display(),
                                err
                            );

                            log_warn(message);
                        }
                    }
                }

                layers
            }
            Err(_) => vec![],
        };

        Layers { layers }
    }
}

#[derive(Debug)]
pub struct RngLayerFile {
    layer: String,
    path: PathBuf,
}

impl RngLayerFile {
    pub fn get_layer(&self) -> &str {
        &self.layer
    }
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug)]
struct Layer {
    _name: String,
    files: Vec<PathBuf>,
}

impl Layer {
    fn try_from_dir_entry(dir_entry: &DirEntry) -> Result<Layer> {
        let context = format!("try to create layer from dir entry: {:#?}", dir_entry);
        let name =
            try_convert_os_string_to_string(dir_entry.file_name()).context(context.clone())?;
        let mut files = vec![];

        read_dir(dir_entry.path())
            .context(context)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .for_each(|e| files.push(e.path()));

        Ok(Layer { _name: name, files })
    }
}

fn try_convert_os_string_to_string(string: OsString) -> Result<String> {
    let converted = string
        .to_str()
        .context("try to convert os string to string")?
        .to_string();

    Ok(converted)
}

fn sort_utility(mut files: Vec<RngLayerFile>) -> Vec<RngLayerFile> {
    files.sort_by(|a, b| a.path.cmp(&b.path));

    files
}
