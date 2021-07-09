use std::{
    borrow::Borrow,
    collections::HashMap,
    convert::TryFrom,
    hash::Hash,
    io::{self, Read, Write},
    iter::FromIterator,
    string::FromUtf8Error,
};

use crate::{
    error::Error,
    variant::{self, VariantType},
};

use thiserror::Error;

use variant::Variant;

mod reader;
mod writer;

use reader::read_attributes;

use writer::write_attributes;

pub(crate) type AttributeData = HashMap<String, Variant>;

macro_rules! create_attribute_type {
    ({
        $(
            $key:ident = $number:literal,
        )+
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) enum AttributeType {
            $(
                $key = $number,
            )+
        }

        impl TryFrom<VariantType> for AttributeType {
            type Error = AttributeError;

            fn try_from(variant_type: VariantType) -> Result<Self, Self::Error> {
                match variant_type {
                    $(
                        VariantType::$key => Ok(AttributeType::$key),
                    )+

                    _ => Err(AttributeError::InvalidVariantType),
                }
            }
        }

        impl TryFrom<u8> for AttributeType {
            type Error = AttributeError;

            fn try_from(byte: u8) -> Result<Self, Self::Error> {
                match byte {
                    $(
                        $number => Ok(Self::$key),
                    )+

                    other => Err(AttributeError::InvalidValueType(other))
                }
            }
        }
    };
}

create_attribute_type!({
    // ??? = 0x01,
    BinaryString = 0x02,
    Bool = 0x03,
    // ??? = 0x04,
    Float32 = 0x05,
    Float64 = 0x06,
    // ??? = 0x07,
    // ??? = 0x08,
    UDim = 0x09,
    UDim2 = 0x0A,
    // ??? = 0x0B,
    // ??? = 0x0C,
    // ??? = 0x0D,
    BrickColor = 0x0E,
    Color3 = 0x0F,
    Vector2 = 0x10,
    Vector3 = 0x11,
    // ??? = 0x12,
    // ??? = 0x13,
    // ??? = 0x14,
    // ??? = 0x15,
    // ??? = 0x16,
    NumberSequence = 0x17,
    // ??? = 0x18,
    ColorSequence = 0x19,
    // ??? = 0x1A,
    NumberRange = 0x1B,
    Rect = 0x1C,
});

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Attributes {
    data: AttributeData,
}

impl Attributes {
    /// Creates an empty `Attributes` struct
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads from a serialized attributes string, and produces a new `Attributes` from it.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        Ok(Attributes {
            data: read_attributes(reader)?,
        })
    }

    /// Writes the attributes as a serialized string to the writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        write_attributes(&self.data, &mut writer).map_err(Into::into)
    }

    /// Get the attribute with the following key.
    pub fn get<K: Borrow<str>>(&self, key: K) -> Option<&Variant> {
        self.data.get(key.borrow())
    }

    /// Inserts an attribute with the given key and value.
    /// Will return the attribute that used to be there if one existed.
    pub fn insert(&mut self, key: String, value: Variant) -> Option<Variant> {
        self.data.insert(key, value)
    }

    /// Removes an attribute with the given key.
    /// Will return the value that was there if one existed.
    pub fn remove<K: Hash + Eq + Borrow<str>>(&mut self, key: K) -> Option<Variant> {
        self.data.remove(key.borrow())
    }

    /// Returns an iterator of borrowed attributes.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Variant)> {
        self.data.iter()
    }
}

impl IntoIterator for Attributes {
    type IntoIter = AttributesIntoIter;
    type Item = (String, Variant);

    fn into_iter(self) -> Self::IntoIter {
        AttributesIntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl FromIterator<(String, Variant)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (String, Variant)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

/// An owning iterator over the entries of an `Attributes`.
/// This is created by [`Attributes::into_iter`].
pub struct AttributesIntoIter {
    iter: std::collections::hash_map::IntoIter<String, Variant>,
}

impl Iterator for AttributesIntoIter {
    type Item = (String, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, Error)]
pub(crate) enum AttributeError {
    #[error("invalid value type: {0}")]
    InvalidValueType(u8),

    #[error("invalid brick color: {0}")]
    InvalidBrickColor(u32),

    #[error("invalid name")]
    InvalidName,

    #[error("invalid size")]
    InvalidSize,

    #[error("invalid variant type passed")]
    InvalidVariantType,

    #[error("no value type was found")]
    NoValueType,

    #[error("malformed attribute key")]
    MalformedEntryKey(FromUtf8Error),

    #[error("couldn't write to writer")]
    ToWriterFail(
        #[from]
        #[source]
        io::Error,
    ),

    #[error("couldn't read bytes to deserialize {0}")]
    Other(&'static str),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[cfg(feature = "serde")]
    mod snapshot_tests {
        use super::*;

        // This is taken from rbx-test-files/models/attributes/xml.rbxmx, but with the NaN removed.
        // This is pasted raw as to not create a circular dependency in test (rbx_types -> rbx_xml/rbx_binary -> rbx_types)
        const ATTRIBUTES_BASE64: &str = "\
        DgAAAAYAAABTdHJpbmcCDQAAAEhlbGxvLCB3b3JsZCEHAAAAQm9vbGVhbgMBBgAAAE51bWJl\
        cgYAAAAAgBzIQAsAAABOdW1iZXJSYW5nZRsAAKBAAAAgQQQAAABVRGltCQAAAD9kAAAABQAA\
        AFVEaW0yCgAAAD8KAAAAMzMzPx4AAAAEAAAAUmVjdBwAAIA/AAAAQAAAQEAAAIBACgAAAEJy\
        aWNrQ29sb3IO7AMAAAYAAABDb2xvcjMPo6IiPwAAAAAAAIA/DgAAAE51bWJlclNlcXVlbmNl\
        FwMAAAAAAAAAAAAAAAAAgD8AAAAAAAAAPwAAAAAAAAAAAACAPwAAgD8HAAAAVmVjdG9yMhAA\
        ACBBAABIQgcAAABWZWN0b3IzEQAAgD8AAABAAABAQA0AAABDb2xvclNlcXVlbmNlGQMAAAAA\
        AAAAAAAAAAAAgD8AAAAAAAAAAAAAAAAAAAA/AAAAAAAAgD8AAAAAAAAAAAAAgD8AAAAAAAAA\
        AAAAgD8IAAAASW5maW5pdHkGAAAAAAAA8H8=";

        #[test]
        fn test_round_trip_attributes() {
            let attributes_value =
                base64::decode(ATTRIBUTES_BASE64).expect("bad base64 for attributes");

            let attributes = Attributes::from_reader(&attributes_value[..])
                .expect("couldn't deserialize attributes");

            let attributes_stable_order = BTreeMap::from_iter(attributes.clone().into_iter());
            insta::assert_yaml_snapshot!(attributes_stable_order);

            let mut new_attribute_bytes = Vec::<u8>::new();
            attributes
                .to_writer(&mut new_attribute_bytes)
                .expect("couldn't write attributes to buffer");

            let new_attributes = Attributes::from_reader(new_attribute_bytes.as_slice())
                .expect("couldn't deserialize crate produced binary");

            let new_attributes_stable_order = BTreeMap::from_iter(new_attributes.into_iter());

            assert_eq!(attributes_stable_order, new_attributes_stable_order);
        }
    }

    #[test]
    fn test_attribute_removal() {
        let mut attributes = Attributes::new();
        attributes.insert("key".to_owned(), Variant::Bool(true));
        assert_eq!(attributes.remove("key"), Some(Variant::Bool(true)));
    }
}
