use std::path::Path;

#[derive(Debug)]
pub struct AppConfiguration<L: AsRef<Path>, D: AsRef<Path>> {
    layers_dir: L,
    destination_dir: D,
}

impl<L: AsRef<Path>, D: AsRef<Path>> AppConfiguration<L, D> {
    pub fn new(layers_dir: L, destination_dir: D) -> AppConfiguration<L, D> {
        AppConfiguration {
            layers_dir,
            destination_dir,
        }
    }

    pub fn get_layers_dir(&self) -> &Path {
        self.layers_dir.as_ref()
    }

    pub fn get_destination_dir(&self) -> &Path {
        self.destination_dir.as_ref()
    }
}
