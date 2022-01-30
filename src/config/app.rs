use std::path::Path;

#[derive(Debug)]
pub struct AppConfiguration<L: AsRef<Path>, D: AsRef<Path>> {
    _layers_dir: L,
    _destination_dir: D,
}

impl<L: AsRef<Path>, D: AsRef<Path>> AppConfiguration<L, D> {
    pub fn new(layers_dir: L, destination_dir: D) -> AppConfiguration<L, D> {
        AppConfiguration {
            _layers_dir: layers_dir,
            _destination_dir: destination_dir,
        }
    }
}
