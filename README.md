# Sdx NFT Art Engine

This application is an image processor which uses layers, containing different layer files and creates unique images
based on the defined layer order and layer files.

> IMPORTANT: This project as heavily inspired by the [HashLips ArtEngine](https://github.com/HashLips/hashlips_art_engine).

## Usage example

- Most simple image processing: `$ sdx_ntx_art_collection`
- Using different layer configuration file: `$ sdx_ntx_art_collection -c ./path/to/layer_config.json`
- Cleanup existing processed images in destination direction: `$ sdx_ntx_art_collection --cleanup`
- Using different destination direction: `$ sdx_ntx_art_collection -d ./path/to/destination/direction`
- Change layers directory: `$ sdx_ntx_art_collection -l ./path/to/layer/direction`
- Set max retries to a higher value (Required when a lot of possible combinations
  exists): `$ sdx_ntx_art_collection -m 10000`

### Help output

```text
Sdx NFT Art Engine

USAGE:
    sdx_nft_art_engine.exe [OPTIONS]

OPTIONS:
    -c, --config-file <CONFIG_FILE>
            Configuration file used to create image from layers [default:
            ./config/sample_layer_configs.json]

        --cleanup
            Removes the destination directory and all of the content

    -d, --destination-dir <DESTINATION_DIR>
            Destination directory containing the processed images and metadata [default: ./build]

    -h, --help
            Print help information

    -l, --layer-dir <LAYER_DIR>
            Directory containing the layers and their images [default: ./layers/Example]

    -m, --max-retry <MAX_RETRY>
            How often the algorithm will retry to to create a new image edition of the current layer
            [default: 1000]

    -r, --resize
            Resizes images (currently everything to 512x512px)
```
