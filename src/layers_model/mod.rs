use std::ffi::{OsString};
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::logger::log_warn;

#[derive(Debug)]
pub struct Layers {
    _layers: Vec<Layer>,
}

impl Layers {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Layers {
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

        Layers { _layers: layers }
    }
}

#[derive(Debug)]
struct Layer {
    _name: String,
    _files: Vec<LayerFile>,
}

impl Layer {
    fn try_from_dir_entry(dir_entry: &DirEntry) -> Result<Layer> {
        let context = format!("try to create layer from dir entry: {:#?}", dir_entry);
        let name =
            try_convert_os_string_to_string(dir_entry.file_name()).context(context.clone())?;
        let mut files = vec![];

        for entry in read_dir(dir_entry.path())
            .context(context.clone())?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let file_name = try_convert_os_string_to_string(entry.file_name())?;
            let name = file_name
                .split('.')
                .next()
                .context(context.clone())?
                .to_string();

            files.push(LayerFile::new(file_name, name, entry.path()))
        }

        Ok(Layer {
            _name: name,
            _files: files,
        })
    }
}

#[derive(Debug)]
struct LayerFile {
    _file_name: String,
    _name: String,
    _path: PathBuf,
}

impl LayerFile {
    fn new(file_name: String, name: String, path: PathBuf) -> LayerFile {
        LayerFile {
            _file_name: file_name,
            _name: name,
            _path: path,
        }
    }
}

fn try_convert_os_string_to_string(string: OsString) -> Result<String> {
    let converted = string
        .to_str()
        .context("try to convert os string to string")?
        .to_string();

    Ok(converted)
}
