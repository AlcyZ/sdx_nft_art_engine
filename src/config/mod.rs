use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::hashing::simple_sha256;

#[derive(Debug)]
pub struct Configuration {
    layers_dir: PathBuf,
    destination_dir: PathBuf,
    resize: bool,
    max_tries: u32,
    image_size: u32,
}

impl Configuration {
    pub fn new<L: AsRef<Path>, D: AsRef<Path>>(
        layers_dir: L,
        destination_dir: D,
        resize: bool,
        max_tries: u32,
        image_size: u32,
    ) -> Configuration {
        if !layers_dir.as_ref().is_dir() {
            std::fs::create_dir_all(layers_dir.as_ref()).unwrap();
        }
        if !destination_dir.as_ref().is_dir() {
            std::fs::create_dir_all(destination_dir.as_ref()).unwrap();
        }

        let layers_dir = layers_dir.as_ref().to_path_buf();
        let destination_dir = destination_dir.as_ref().to_path_buf();

        Configuration {
            layers_dir,
            destination_dir,
            resize,
            max_tries,
            image_size,
        }
    }

    pub fn get_layers_dir(&self) -> &Path {
        &self.layers_dir
    }

    pub fn get_destination_dir(&self) -> &Path {
        &self.destination_dir
    }

    pub fn get_max_tries(&self) -> u32 {
        self.max_tries
    }

    pub fn get_image_size(&self) -> u32 {
        self.image_size
    }

    pub fn is_resize_enabled(&self) -> bool {
        self.resize
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerConfigurations {
    _name_prefix: String,
    _description: String,
    _ipfs_uri: String,
    layers: Vec<LayerConfiguration>,
}

impl LayerConfigurations {
    pub fn get_layers(&self) -> &Vec<LayerConfiguration> {
        &self.layers
    }
}

impl LayerConfigurations {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<LayerConfigurations> {
        let context = format!(
            "create layer configurations from file: {}",
            path.as_ref().display()
        );
        let config_content = read_to_string(path).context(context.clone())?;

        serde_json::from_str(&config_content).context(context)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerConfiguration {
    size: u32,
    order: Vec<LayerConfig>,
    #[serde(skip)]
    dna: RefCell<Option<String>>,
}

impl LayerConfiguration {
    pub fn _from_file<P: AsRef<Path>>(path: P) -> Result<LayerConfiguration> {
        let context = format!(
            "create layer configuration from file: {}",
            path.as_ref().display()
        );
        let config_content = read_to_string(path).context(context.clone())?;

        serde_json::from_str(&config_content).context(context)
    }
}

impl LayerConfiguration {
    pub fn _new(size: u32, order: Vec<LayerConfig>) -> LayerConfiguration {
        let chained_order = order
            .iter()
            .map(|o| o.get_name())
            .collect::<Vec<&str>>()
            .join(":");

        let dna = RefCell::new(Some(simple_sha256(chained_order.as_bytes())));

        LayerConfiguration { size, order, dna }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_order(&self) -> &Vec<LayerConfig> {
        &self.order
    }

    pub fn get_dna(&self) -> String {
        let is_dna_empty = self.dna.borrow().as_ref().is_none();

        if is_dna_empty {
            let chained_order = self
                .order
                .iter()
                .map(|o| o.get_name())
                .collect::<Vec<&str>>()
                .join(":");

            *self.dna.borrow_mut() = Some(simple_sha256(chained_order.as_bytes()));
        }

        self.dna.borrow().as_ref().unwrap().to_string()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerConfig {
    name: String,

    #[serde(default = "default_pick")]
    pick_min: u32,
    #[serde(default = "default_pick")]
    pick_max: u32,
}

impl LayerConfig {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_pick_min(&self) -> u32 {
        self.pick_min
    }

    pub fn get_pick_max(&self) -> u32 {
        self.pick_max
    }
}

fn default_pick() -> u32 {
    1
}
