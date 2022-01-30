use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug)]
pub(super) struct LayerMetaData<T: Serialize> {
    name: String,
    description: String,
    image: String,
    attributes: Vec<LayerMetaAttributeDataBackup<T>>,
}

#[derive(Debug)]
pub(super) struct LayerMetaAttributeDataBackup<T: Serialize> {
    trait_type: String,
    value: T,
    display_type: Option<DisplayType>,
}

impl<T: Serialize> LayerMetaAttributeDataBackup<T> {
    fn serialize_attribute<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let (len, display_type) = match &self.display_type {
            Some(display_type) => {
                if let DisplayType::None = display_type {
                    (2, None)
                } else {
                    (3, Some(display_type.to_string()))
                }
            }
            None => (2, None),
        };
        let mut state = serializer.serialize_struct("LayerMetaAttributeData", len)?;

        state.serialize_field("trait_type", &self.trait_type)?;
        state.serialize_field("value", &self.value)?;

        if let Some(display_type) = display_type {
            state.serialize_field("display_type", &display_type)?;
        }

        state.end()
    }
}

impl LayerMetaAttributeDataBackup<String> {
    pub(super) fn new(trait_type: String, value: String) -> LayerMetaAttributeDataBackup<String> {
        LayerMetaAttributeDataBackup {
            trait_type,
            value,
            display_type: None,
        }
    }
}

impl Serialize for LayerMetaAttributeDataBackup<String> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("LayerMetaAttributeData<String>", 2)?;

        state.serialize_field("trait_type", &self.trait_type)?;
        state.serialize_field("value", &self.value)?;

        state.end()
    }
}

impl LayerMetaAttributeDataBackup<u32> {
    pub(super) fn new(
        trait_type: String,
        value: u32,
        display_type: DisplayType,
    ) -> LayerMetaAttributeDataBackup<u32> {
        LayerMetaAttributeDataBackup {
            trait_type,
            value,
            display_type: Some(display_type),
        }
    }
}

impl Serialize for LayerMetaAttributeDataBackup<u32> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        LayerMetaAttributeDataBackup::serialize_attribute(self, serializer)
    }
}

impl LayerMetaAttributeDataBackup<f32> {
    pub(super) fn new(
        trait_type: String,
        value: f32,
        display_type: DisplayType,
    ) -> LayerMetaAttributeDataBackup<f32> {
        LayerMetaAttributeDataBackup {
            trait_type,
            value,
            display_type: Some(display_type),
        }
    }
}

impl Serialize for LayerMetaAttributeDataBackup<f32> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        LayerMetaAttributeDataBackup::serialize_attribute(self, serializer)
    }
}

enum AttributeMeta {
    String(StringAttributeMeta),
    Integer(IntegerAttributeMeta),
    Float(FloatAttributeMeta),
}

struct StringAttributeMeta {
    trait_type: String,
    value: String,
}

impl StringAttributeMeta {
    fn new(trait_type: String, value: String) -> StringAttributeMeta {
        StringAttributeMeta { trait_type, value }
    }
}

struct IntegerAttributeMeta {
    trait_type: String,
    value: i32,
    display_type: DisplayType,
}

impl IntegerAttributeMeta {
    fn new(trait_type: String, value: i32, display_type: DisplayType) -> IntegerAttributeMeta {
        IntegerAttributeMeta {
            trait_type,
            value,
            display_type,
        }
    }
}

struct FloatAttributeMeta {
    trait_type: String,
    value: f32,
    display_type: DisplayType,
}

impl Serialize for FloatAttributeMeta {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}

impl FloatAttributeMeta {
    fn new(trait_type: String, value: f32, display_type: DisplayType) -> FloatAttributeMeta {
        FloatAttributeMeta {
            trait_type,
            value,
            display_type,
        }
    }
}

#[derive(Debug)]
enum DisplayType {
    None,
    Number,
    BoostPercentage,
    BoostNumber,
}

impl ToString for DisplayType {
    fn to_string(&self) -> String {
        match self {
            DisplayType::None => "".into(),
            DisplayType::Number => "number".into(),
            DisplayType::BoostPercentage => "boost_percentage".into(),
            DisplayType::BoostNumber => "boost_number".into(),
        }
    }
}
