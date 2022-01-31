use std::fs::read_to_string;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionConfiguration {
    _name: String,
    _description: String,
    _ipfs_uri: String,
    layers: Vec<LayerConfiguration>,
}

impl EditionConfiguration {
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<EditionConfiguration> {
        let context = format!(
            "Try to create EditionConfiguration from path: {}",
            path.as_ref().display()
        );

        if !path.as_ref().is_file() {
            bail!("Given path ({}) is not a file!", path.as_ref().display());
        }
        let content = read_to_string(path).context(context.clone())?;
        let config: EditionConfiguration = serde_json::from_str(&content).context(context)?;

        Ok(config)
    }
}

impl EditionConfiguration {
    pub fn _get_name(&self) -> &str {
        &self._name
    }
    pub fn _get_description(&self) -> &str {
        &self._description
    }
    pub fn _get_ipfs_uri(&self) -> &str {
        &self._ipfs_uri
    }
    pub fn get_layers(&self) -> &Vec<LayerConfiguration> {
        &self.layers
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerConfiguration {
    _size: u32,
    order: Vec<LayerOrderConfiguration>,
}

impl LayerConfiguration {
    pub fn _get_size(&self) -> u32 {
        self._size
    }

    pub fn get_order(&self) -> &Vec<LayerOrderConfiguration> {
        &self.order
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerOrderConfiguration {
    name: String,
    #[serde(default = "default_pick")]
    pick_min: u32,
    #[serde(default = "default_pick")]
    pick_max: u32,
}

impl LayerOrderConfiguration {
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
