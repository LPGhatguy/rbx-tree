use std::{
    borrow::Borrow,
    convert::TryFrom,
    io::{self, Write},
};

use super::{AttributeData, AttributeError, AttributeType};

use crate::{
    basic_types::{Color3, UDim, Vector2},
    variant::Variant,
};

fn write_f32<W: Write>(mut writer: W, n: f32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_f64<W: Write>(mut writer: W, n: f64) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_u32<W: Write>(mut writer: W, n: u32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_color3<W: Write>(mut writer: W, color: Color3) -> io::Result<()> {
    write_f32(&mut writer, color.r)?;
    write_f32(&mut writer, color.g)?;
    write_f32(&mut writer, color.b)
}

fn write_string<T: AsRef<[u8]>, W: Write>(mut writer: W, string: T) -> io::Result<()> {
    let bytes = string.as_ref();
    write_u32(&mut writer, bytes.len() as u32)?;
    writer.write_all(bytes)
}

fn write_udim<W: Write>(mut writer: W, udim: UDim) -> io::Result<()> {
    write_f32(&mut writer, udim.scale)?;
    writer.write_all(&udim.offset.to_le_bytes()[..])
}

fn write_vector2<W: Write>(mut writer: W, vector2: Vector2) -> io::Result<()> {
    write_f32(&mut writer, vector2.x)?;
    write_f32(&mut writer, vector2.y)
}

/// Writes the attribute property (AttributesSerialize) from a map of attribute names -> values.
pub(crate) fn write_attributes<W: Write>(
    map: &AttributeData,
    mut writer: W,
) -> Result<(), AttributeError> {
    writer.write_all(&(map.len() as u32).to_le_bytes())?;

    for (name, variant) in map {
        let variant = variant.borrow();

        write_string(&mut writer, &name)?;

        let attribute_type = AttributeType::try_from(variant.ty())?;
        writer.write_all(&[attribute_type as u8])?;

        match (attribute_type, variant) {
            (AttributeType::Bool, Variant::Bool(bool)) => writer.write_all(&[*bool as u8])?,
            (AttributeType::BrickColor, Variant::BrickColor(color)) => {
                write_u32(&mut writer, *color as u32)?
            }
            (AttributeType::Color3, Variant::Color3(color)) => write_color3(&mut writer, *color)?,
            (AttributeType::ColorSequence, Variant::ColorSequence(sequence)) => {
                write_u32(&mut writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut writer, 0.0)?; // Envelope
                    write_f32(&mut writer, keypoint.time)?;
                    write_color3(&mut writer, keypoint.color)?;
                }
            }
            (AttributeType::Float32, Variant::Float32(float)) => write_f32(&mut writer, *float)?,
            (AttributeType::Float64, Variant::Float64(float)) => write_f64(&mut writer, *float)?,
            (AttributeType::NumberRange, Variant::NumberRange(range)) => {
                write_f32(&mut writer, range.min)?;
                write_f32(&mut writer, range.max)?;
            }
            (AttributeType::NumberSequence, Variant::NumberSequence(sequence)) => {
                write_u32(&mut writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut writer, keypoint.envelope)?;
                    write_f32(&mut writer, keypoint.time)?;
                    write_f32(&mut writer, keypoint.value)?;
                }
            }
            (AttributeType::Rect, Variant::Rect(rect)) => {
                write_vector2(&mut writer, rect.min)?;
                write_vector2(&mut writer, rect.max)?
            }
            (AttributeType::BinaryString, Variant::BinaryString(string)) => {
                write_string(&mut writer, string)?
            }
            (AttributeType::UDim, Variant::UDim(udim)) => write_udim(&mut writer, *udim)?,
            (AttributeType::UDim2, Variant::UDim2(udim2)) => {
                write_udim(&mut writer, udim2.x)?;
                write_udim(&mut writer, udim2.y)?
            }
            (AttributeType::Vector2, Variant::Vector2(vector2)) => {
                write_vector2(&mut writer, *vector2)?
            }
            (AttributeType::Vector3, Variant::Vector3(vector3)) => {
                write_f32(&mut writer, vector3.x)?;
                write_f32(&mut writer, vector3.y)?;
                write_f32(&mut writer, vector3.z)?
            }

            (other_type, other_variant) => unreachable!(
                "variant {:?} didn't match attribute type {:?}",
                other_variant, other_type
            ),
        }
    }

    Ok(())
}
