use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};

use crate::config::{Configuration, LayerConfigurations};
use crate::logger::{log_measure, log_warn};
use crate::processor::layers::Layers;

pub fn run() -> Result<()> {
    let context = "Execute application";

    let matches = get_matches();

    let layers_dir = matches.value_of("layer").unwrap();
    let destination_dir = matches.value_of("destination").unwrap();
    let layer_config_file = matches.value_of("config").unwrap();
    let resize = matches.is_present("resize");
    let size = match matches.value_of("size").unwrap().parse::<u32>() {
        Ok(size) => size,
        Err(err) => {
            log_warn(format!(
                "Invalid 'size' argument provided, use default of 512 ({}).",
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

    if matches.is_present("cleanup") {
        if let Err(err) = std::fs::remove_dir_all(&destination_dir) {
            let message = format!(
                "Failed to truncate destination directory ({}) - {}",
                &destination_dir, err
            );
            log_warn(message);
        }
    }

    let layer_configurations =
        LayerConfigurations::from_file(layer_config_file).context(context)?;
    let config = Configuration::new(layers_dir, destination_dir, resize, max_tries, size);
    let layers = Layers::from_configuration(&config).context(context)?;

    let log = log_measure("Processing layers");
    for layer_config in layer_configurations.get_layers() {
        layers
            .create_images(layer_config, &config)
            .context(context)?;
    }
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
                .help("Configuration file used to create image from layers")
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
            Arg::new("resize")
                .help("Resizes images (currently everything to 512x512px)")
                .short('r')
                .long("resize"),
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
                .long("cleanup"))
        .arg(
            Arg::new("size")
                .help("Image size (in px) of processed images")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .takes_value(true)
                .default_value("512"),
        )
        .get_matches()
}
