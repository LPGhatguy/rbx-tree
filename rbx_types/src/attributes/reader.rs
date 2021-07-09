// Thanks to Anaminus! https://github.com/RobloxAPI/rbxattr/blob/master/spec.md

use std::{
    collections::HashMap,
    io::{self, Read},
};

use crate::{
    basic_types::{
        Color3, ColorSequence, ColorSequenceKeypoint, NumberRange, NumberSequence,
        NumberSequenceKeypoint, Rect, UDim, UDim2, Vector2, Vector3,
    },
    brick_color::BrickColor,
    variant::Variant,
    BinaryString,
};

use super::*;

fn read_u8<R: Read>(mut reader: R) -> io::Result<u8> {
    let mut bytes = [0u8; 1];
    reader.read_exact(&mut bytes)?;
    Ok(bytes[0])
}

fn read_i32<R: Read>(mut reader: R) -> io::Result<i32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(i32::from_le_bytes(bytes))
}

fn read_u32<R: Read>(mut reader: R) -> io::Result<u32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_f32<R: Read>(mut reader: R) -> io::Result<f32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(f32::from_le_bytes(bytes))
}

fn read_f64<R: Read>(mut reader: R) -> io::Result<f64> {
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes)?;
    Ok(f64::from_le_bytes(bytes))
}

fn read_string<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let size = read_u32(&mut reader)? as usize;
    let mut characters = vec![0u8; size];
    reader.read_exact(&mut characters)?;
    Ok(characters)
}

fn read_color3<R: Read>(mut reader: R) -> io::Result<Color3> {
    Ok(Color3::new(
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
    ))
}

fn read_udim<R: Read>(mut reader: R) -> io::Result<UDim> {
    Ok(UDim::new(read_f32(&mut reader)?, read_i32(&mut reader)?))
}

fn read_vector2<R: Read>(mut reader: R) -> io::Result<Vector2> {
    Ok(Vector2::new(read_f32(&mut reader)?, read_f32(&mut reader)?))
}

/// Reads through an attribute property (AttributesSerialize) and returns a map of attribute names -> values.
pub(crate) fn read_attributes<R: Read>(
    mut value: R,
) -> Result<HashMap<String, Variant>, AttributeError> {
    let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;
    let mut attributes = HashMap::with_capacity(size as usize);

    for _ in 0..size {
        let name =
            String::from_utf8(read_string(&mut value).map_err(|_| AttributeError::InvalidName)?)
                .map_err(AttributeError::MalformedEntryKey)?;

        let attribute_type =
            AttributeType::try_from(read_u8(&mut value).map_err(|_| AttributeError::NoValueType)?)?;

        let value = match attribute_type {
            AttributeType::BrickColor => {
                let color =
                    read_u32(&mut value).map_err(|_| AttributeError::Other("BrickColor"))?;

                BrickColor::from_number(color as u16)
                    .ok_or(AttributeError::InvalidBrickColor(color))?
                    .into()
            }

            AttributeType::Bool => {
                (read_u8(&mut value).map_err(|_| AttributeError::Other("bool"))? != 0).into()
            }

            AttributeType::Color3 => read_color3(&mut value)
                .map_err(|_| AttributeError::Other("Color3"))?
                .into(),

            AttributeType::ColorSequence => {
                let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is always zero and can be ignored.
                    let _envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint time"))?;

                    let color = read_color3(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint color"))?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                ColorSequence { keypoints }.into()
            }

            AttributeType::Float32 => read_f32(&mut value)
                .map_err(|_| AttributeError::Other("float32"))?
                .into(),

            AttributeType::Float64 => read_f64(&mut value)
                .map_err(|_| AttributeError::Other("float64"))?
                .into(),

            AttributeType::NumberRange => NumberRange::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("NumberRange min"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("NumberRange max"))?,
            )
            .into(),

            AttributeType::NumberSequence => {
                let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence time"))?;

                    let value = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                NumberSequence { keypoints }.into()
            }

            AttributeType::Rect => Rect::new(
                read_vector2(&mut value).map_err(|_| AttributeError::Other("Rect min"))?,
                read_vector2(&mut value).map_err(|_| AttributeError::Other("Rect max"))?,
            )
            .into(),

            AttributeType::BinaryString => {
                let binary_string: BinaryString = read_string(&mut value)
                    .map_err(|_| AttributeError::Other("string"))?
                    .into();
                binary_string.into()
            }

            AttributeType::UDim => read_udim(&mut value)
                .map_err(|_| AttributeError::Other("UDim"))?
                .into(),

            AttributeType::UDim2 => UDim2::new(
                read_udim(&mut value).map_err(|_| AttributeError::Other("UDim2 X"))?,
                read_udim(&mut value).map_err(|_| AttributeError::Other("UDim2 Y"))?,
            )
            .into(),

            AttributeType::Vector2 => Vector2::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector2 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector2 Y"))?,
            )
            .into(),

            AttributeType::Vector3 => Vector3::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 Y"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 Z"))?,
            )
            .into(),
        };

        attributes.insert(name, value);
    }

    Ok(attributes)
}
