use serde::Serialize;

use crate::config::edition::EditionConfiguration;
use crate::processor::model::image::ImageFile;

#[derive(Debug, Serialize)]
pub(super) struct Meta {
    name: String,
    description: String,
    image: String,
    attributes: Vec<MetaAttribute>,
}

impl Meta {
    pub(super) fn new(
        edition: u32,
        edition_config: &EditionConfiguration,
        image_files: &[ImageFile],
    ) -> Meta {
        let name = edition_config._get_name().to_string() + &format!(" #{}", edition);
        let image = edition_config._get_ipfs_uri().to_string() + &format!("{}.png", edition);
        let attributes = image_files.iter().map(MetaAttribute::from).collect();

        Meta {
            name,
            description: edition_config._get_description().to_string(),
            image,
            attributes,
        }
    }
}

#[derive(Debug, Serialize)]
struct MetaAttribute {
    trait_type: String,
    value: String,
}

impl From<&ImageFile> for MetaAttribute {
    fn from(image_file: &ImageFile) -> Self {
        MetaAttribute {
            trait_type: image_file.get_layer().to_string(),
            value: image_file.get_name().to_string(),
        }
    }
}
