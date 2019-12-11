//! Deserializer that reads a file and creates a debug representation of it.
//! It's intended to be used to snapshot test the binary serializer without
//! suffering from same-inverse-bug problems.

use std::io::Read;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{chunk::Chunk, deserializer::FileHeader};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedModel {
    pub num_types: u32,
    pub num_instances: u32,
    pub chunks: Vec<DecodedChunk>,
}

impl DecodedModel {
    pub fn from_reader<R: Read>(mut reader: R) -> Self {
        let header = FileHeader::decode(&mut reader).expect("invalid file header");
        let mut chunks = Vec::new();

        loop {
            let chunk = Chunk::decode(&mut reader).expect("invalid chunk");

            match &chunk.name {
                b"META" => {
                    chunks.push(DecodedChunk::Meta {
                        contents: chunk.data.into(),
                    });
                }
                b"INST" => {
                    chunks.push(DecodedChunk::Inst {
                        contents: chunk.data.into(),
                    });
                }
                b"PROP" => {
                    chunks.push(DecodedChunk::Prop {
                        contents: chunk.data.into(),
                    });
                }
                b"PRNT" => {
                    chunks.push(DecodedChunk::Prnt {
                        contents: chunk.data.into(),
                    });
                }
                b"END\0" => {
                    chunks.push(DecodedChunk::End);
                    break;
                }
                _ => {
                    chunks.push(DecodedChunk::Unknown {
                        name: String::from_utf8_lossy(&chunk.name[..]).to_string(),
                    });
                }
            }
        }

        DecodedModel {
            num_types: header.num_types,
            num_instances: header.num_instances,
            chunks,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DecodedChunk {
    Meta { contents: UnknownBuffer },

    Inst { contents: UnknownBuffer },

    Prop { contents: UnknownBuffer },

    Prnt { contents: UnknownBuffer },

    End,

    Unknown { name: String },
}

/// Contains data that we haven't decoded for a chunk. Using `UnknownBuffer`
/// should generally be a placeholder since it's results are opaque, but stable.
#[derive(Debug)]
pub struct UnknownBuffer {
    contents: Vec<u8>,
}

impl From<Vec<u8>> for UnknownBuffer {
    fn from(contents: Vec<u8>) -> Self {
        Self { contents }
    }
}

impl Serialize for UnknownBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&base64::display::Base64Display::with_config(
            &self.contents,
            base64::STANDARD,
        ))
    }
}

impl<'de> Deserialize<'de> for UnknownBuffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = <&str>::deserialize(deserializer)?;
        let contents = base64::decode(encoded).map_err(serde::de::Error::custom)?;

        Ok(UnknownBuffer { contents })
    }
}
