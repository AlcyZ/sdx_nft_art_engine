use std::path::Path;

use clap::ArgMatches;

use crate::logger::log_warn;

#[derive(Debug)]
pub struct AppConfiguration<L: AsRef<Path>, D: AsRef<Path>> {
    layers_dir: L,
    destination_dir: D,
    size: u32,
    max_tries: u32,
    cleanup: bool,
}

impl AppConfiguration<&str, &str> {
    pub fn from_arg_matches(matches: &ArgMatches) -> AppConfiguration<&str, &str> {
        let layers_dir = matches.value_of("layer").unwrap();
        let destination_dir = matches.value_of("destination").unwrap();
        let size = match matches.value_of("size").unwrap().parse::<u32>() {
            Ok(size) => size,
            Err(err) => {
                log_warn(format!(
                    "Invalid 'size' argument provided, use default of 1024 ({}).",
                    err
                ));

                512
            }
        };
        let max_tries = match matches.value_of("max-retry").unwrap().parse::<u32>() {
            Ok(max_tries) => max_tries,
            Err(err) => {
                log_warn(format!(
                    "Invalid 'max-retry' argument provided, use default of 1000 ({}).",
                    err
                ));

                1000
            }
        };

        AppConfiguration {
            layers_dir,
            destination_dir,
            size,
            max_tries,
            cleanup: matches.is_present("cleanup"),
        }
    }
}

impl<L: AsRef<Path>, D: AsRef<Path>> AppConfiguration<L, D> {
    pub fn get_layers_dir(&self) -> &Path {
        self.layers_dir.as_ref()
    }

    pub fn get_destination_dir(&self) -> &Path {
        self.destination_dir.as_ref()
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_max_tries(&self) -> u32 {
        self.max_tries
    }

    pub fn is_cleanup_enabled(&self) -> bool {
        self.cleanup
    }
}
