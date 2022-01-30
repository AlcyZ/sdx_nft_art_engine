use anyhow::Result;

use crate::config::app::AppConfiguration;
use crate::config::edition::EditionConfiguration;
use crate::hashing::simple_sha256;
use crate::logger::{log_info, log_measure, log_warn};

pub fn run() -> Result<()> {
    let test_hash = simple_sha256("foo".as_bytes());
    eprintln!("test_hash = {:#?}", test_hash);
    log_warn("foo");
    log_info("bar");
    let log = log_measure("baz");
    log.finish();

    let edition_configuration =
        EditionConfiguration::try_from_path("./config/layer_configuration.json")?;
    eprintln!("edition_configuration = {:#?}", edition_configuration);

    let app_configuration = AppConfiguration::new("./layers", "./build");
    eprintln!("app_configuration = {:#?}", app_configuration);

    Ok(())
}
