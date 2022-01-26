struct _ImageMetaData {
    name: String,
    description: String,
    image: String,
    dna: String,
    edition: String,
    date: u64,
    attributes: Vec<_ImageMetaDataAttributes>,
}

impl _ImageMetaData {
    // fn new(name: String,
    //        description: String,
    //        image: String,
    //        dna: String,
    //        edition: String,
    //        date: u64,
    //        attributes: Vec<ImageMetaDataAttributes>) -> ImageMetaData {
    //     ImageMetaData {
    //         name,
    //         description,
    //         image,
    //         dna,
    //         edition,
    //         date,
    //         attributes,
    //     }
    // }

    fn _from_layer_composite() {
        todo!("implement it later on");
    }
}

struct _ImageMetaDataAttributes {
    trait_type: String,
    value: String,
}
