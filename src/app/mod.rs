use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};

use crate::config::app::AppConfiguration;
use crate::config::edition::EditionConfiguration;
use crate::layers_model::Layers;
use crate::logger::log_measure;
use crate::processor::create_images;

pub fn run() -> Result<()> {
    let context = "Run application";

    let matches = get_matches();
    let layer_config_file = matches.value_of("config").unwrap();

    let edition_config = EditionConfiguration::try_from_path(layer_config_file).context(context)?;
    let app_config = AppConfiguration::from_arg_matches(&matches);
    let layers = Layers::from_config(&app_config);

    let log = log_measure("create images");
    create_images(&layers, &edition_config, &app_config).context("asd")?;
    log.finish();

    Ok(())
}

fn get_matches() -> ArgMatches {
    App::new("Sdx NFT Art Engine")
        .arg(
            Arg::new("layer")
                .help("Directory containing the layers and their images")
                .short('l')
                .long("layer-dir")
                .value_name("LAYER_DIR")
                .takes_value(true)
                .default_value("./layers"),
        )
        .arg(
            Arg::new("config")
                .help("Edition configuration file used to create image from layers")
                .short('c')
                .long("config-file")
                .value_name("CONFIG_FILE")
                .takes_value(true)
                .default_value("./config/layer_configuration.json"),
        )
        .arg(
            Arg::new("destination")
                .help("Destination directory containing the processed images and metadata")
                .short('d')
                .long("destination-dir")
                .value_name("DESTINATION_DIR")
                .takes_value(true)
                .default_value("./build"),
        )
        .arg(
            Arg::new("max-retry")
                .help("How often the algorithm will retry to to create a new image edition of the current layer")
                .short('m')
                .long("max-retry")
                .value_name("MAX_RETRY")
                .takes_value(true)
                .default_value("1000"),
        )
        .arg(
            Arg::new("cleanup")
                .help("Removes the destination directory and all of the content")
                .short('t')
                .long("cleanup"))
        .arg(
            Arg::new("size")
                .help("Image size (in px) of processed images")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .takes_value(true)
                .default_value("1024"),
        )
        .get_matches()
}
