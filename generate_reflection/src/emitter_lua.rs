use std::{
    io::{self, Write, BufWriter},
    fs::File,
    path::Path,
};

use lazy_static::lazy_static;
use regex::Regex;
use rbx_dom_weak::RbxValue;

use crate::{
    database::ReflectionDatabase,
    api_dump::{DumpClass, DumpClassMember},
};

lazy_static! {
    static ref LUA_IDENT: Regex = Regex::new("^[a-zA-Z_]+[a-zA-Z0-9_]*$").unwrap();
}

pub fn emit(database: &ReflectionDatabase, output_dir: &Path) -> io::Result<()> {
    let output_path = output_dir.join("dump.lua");

    let mut file = BufWriter::new(File::create(&output_path)?);
    writeln!(file, "-- This file is automatically generated.")?;

    writeln!(file, "local classes = {{")?;
    for class in &database.dump.classes {
        emit_class(&mut file, class)?;
    }
    writeln!(file, "}}")?;

    emit_defaults(&mut file, database)?;

    writeln!(file, "return {{")?;
    writeln!(file, "\tclasses = classes,")?;
    writeln!(file, "\tdefaults = defaults,")?;
    writeln!(file, "}}")?;

    Ok(())
}

fn emit_defaults<W: Write>(output: &mut W, database: &ReflectionDatabase) -> io::Result<()> {
    writeln!(output, "local defaults = {{")?;
    for (instance_name, instance_properties) in &database.default_properties {
        writeln!(output, "\t{} = {{", instance_name)?;

        for (property_name, default_value) in instance_properties {
            if !LUA_IDENT.is_match(property_name) {
                continue;
            }

            write!(output, "\t\t{} = ", property_name)?;
            emit_value(output, default_value)?;
            writeln!(output, ",")?;
        }

        writeln!(output, "\t}},")?;
    }
    writeln!(output, "}}")?;


    Ok(())
}

fn emit_class<W: Write>(output: &mut W, class: &DumpClass) -> io::Result<()> {
    writeln!(output, "\t{} = {{", class.name)?;

    if class.superclass != "<<<ROOT>>>" {
        writeln!(output, "\t\tsuperclass = \"{}\",", class.superclass)?;
    }

    writeln!(output, "\t\tproperties = {{")?;
    for member in &class.members {
        match member {
            DumpClassMember::Property(property) => {
                if !LUA_IDENT.is_match(&property.name) {
                    continue;
                }

                writeln!(output, "\t\t\t{} = {{", property.name)?;
                writeln!(output, "\t\t\t\ttype = \"{}\",", property.value_type.name)?;

                write!(output, "\t\t\t\ttags = {{")?;

                for tag in &property.tags {
                    write!(output, "{} = true, ", tag)?;
                }

                writeln!(output, "}},")?;

                writeln!(output, "\t\t\t\tcanSave = {},", property.serialization.can_save)?;
                writeln!(output, "\t\t\t\tcanLoad = {},", property.serialization.can_load)?;

                writeln!(output, "\t\t\t}},")?;
            },
            _ => {}
        }
    }
    writeln!(output, "\t\t}},")?;

    writeln!(output, "\t}},")?;
    Ok(())
}

fn emit_value<W: Write>(output: &mut W, value: &RbxValue) -> io::Result<()> {
    use RbxValue::*;

    match value {
        BinaryString { value } => {
            output.write_all(b"\"")?;
            output.write_all(value)?;
            output.write_all(b"\"")?;
            Ok(())
        }
        Bool { value } => write!(output, "{}", *value),
        CFrame { value } => {
            write!(output,
                "CFrame.new({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                value[0], value[1], value[2],
                value[3], value[4], value[5],
                value[6], value[7], value[8],
                value[9], value[10], value[11])
        }
        Color3 { value } => write!(output, "Color3.new({}, {}, {})", value[0], value[1], value[2]),
        Color3uint8 { value } => write!(output, "Color3.fromRGB({}, {}, {})", value[0], value[1], value[2]),
        Content { value } => write!(output, "\"{}\"", value),
        Enum { value } => write!(output, "{}", value),
        Float32 { value } => write!(output, "{}", value),
        Float64 { value } => write!(output, "{}", value),
        Int32 { value } => write!(output, "{}", value),
        Int64 { value } => write!(output, "{}", value),
        NumberRange { value } => write!(output, "NumberRange.new({}, {})", value.0, value.1),
        NumberSequence { value } => {
            write!(output, "NumberSequence.new(")?;

            for (index, keypoint) in value.keypoints.iter().enumerate() {
                write!(output, "NumberSequenceKeypoint.new({}, {}, {})",
                    keypoint.time, keypoint.value, keypoint.envelope)?;

                if index < value.keypoints.len() - 1 {
                    write!(output, ", ")?;
                }
            }

            write!(output, ")")
        }
        ColorSequence { value } => {
            write!(output, "ColorSequence.new(")?;

            for (index, keypoint) in value.keypoints.iter().enumerate() {
                write!(output, "ColorSequenceKeypoint.new({}, Color3.new({}, {}, {}))",
                    keypoint.time, keypoint.color[0], keypoint.color[1], keypoint.color[2])?;

                if index < value.keypoints.len() - 1 {
                    write!(output, ", ")?;
                }
            }

            write!(output, ")")
        }
        Rect { value } => {
            write!(output, "Rect.new({}, {}, {}, {})", value.min.0, value.min.1, value.max.0, value.max.1)
        }
        PhysicalProperties { value } => {
            match value {
                Some(props) => {
                    write!(output, "PhysicalProperties.new({}, {}, {}, {}, {})",
                        props.density, props.friction, props.elasticity, props.friction_weight, props.elasticity_weight)
                }
                None => write!(output, "nil")
            }
        }
        Ref { value } => {
            if value.is_some() {
                panic!("Can't serialize non-None Ref");
            }

            write!(output, "nil")
        }
        String { value } => write!(output, "\"{}\"", value),
        UDim { value } => write!(output, "UDim.new({}, {})", value.0, value.1),
        UDim2 { value } => write!(output, "UDim2.new({}, {}, {}, {})", value.0, value.1, value.2, value.3),
        Vector2 { value } => write!(output, "Vector2.new({}, {})", value[0], value[1]),
        Vector2int16 { value } => write!(output, "Vector2int16.new({}, {})", value[0], value[1]),
        Vector3 { value } => write!(output, "Vector3.new({}, {}, {})", value[0], value[1], value[2]),
        Vector3int16 { value } => write!(output, "Vector3int16.new({}, {}, {})", value[0], value[1], value[2]),
        _ => unimplemented!()
    }
}