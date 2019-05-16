use std::{
    borrow::Cow,
    collections::HashMap,
    io::Write,
};

use rbx_reflection::RbxPropertyTypeDescriptor;
use rbx_dom_weak::{RbxTree, RbxValue, RbxValueType, RbxId, RbxValueConversion};

use crate::{
    core::find_serialized_property_descriptor,
    types::{write_value_xml, write_ref},
    error::{EncodeError as NewEncodeError, EncodeErrorKind},
};

use crate::serializer_core::{XmlEventWriter, XmlWriteEvent};

pub fn encode_internal<W: Write>(output: W, tree: &RbxTree, ids: &[RbxId], options: EncodeOptions) -> Result<(), NewEncodeError> {
    let mut writer = XmlEventWriter::from_output(output);
    let mut state = EmitState::new(options);

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))?;

    for id in ids {
        serialize_instance(&mut writer, &mut state, tree, *id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

/// Describes the strategy that rbx_xml should use when serializing properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodePropertyBehavior {
    /// Ignores properties that aren't known by rbx_xml.
    ///
    /// This is the default.
    IgnoreUnknown,

    /// Write unrecognized properties.
    ///
    /// With this option set, properties that are newer than rbx_xml's
    /// reflection database will show up. It may be problematic to depend on
    /// these properties, since rbx_xml may start supporting them with
    /// non-reflection specific names at a future date.
    WriteUnknown,

    /// Returns an error if any properties are found that aren't known by
    /// rbx_xml.
    ErrorOnUnknown,

    /// Completely turns off rbx_xml's reflection database. Property names and
    /// types will appear exactly as they are in the tree.
    ///
    /// This setting is useful for debugging the model format. It leaves the
    /// user to deal with oddities like how `Part.FormFactor` is actually
    /// serialized as `Part.formFactorRaw`.
    NoReflection,

    #[doc(hidden)]
    __Nonexhaustive,
}

/// Options available for serializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct EncodeOptions {
    property_behavior: EncodePropertyBehavior,
}

impl EncodeOptions {
    /// Constructs a `EncodeOptions` with all values set to their defaults.
    #[inline]
    pub fn new() -> Self {
        EncodeOptions {
            property_behavior: EncodePropertyBehavior::IgnoreUnknown,
        }
    }

    /// Determines how rbx_xml will serialize properties, especially unknown
    /// ones.
    #[inline]
    pub fn property_behavior(self, property_behavior: EncodePropertyBehavior) -> Self {
        EncodeOptions {
            property_behavior,
            ..self
        }
    }

    pub(crate) fn use_reflection(&self) -> bool {
        self.property_behavior != EncodePropertyBehavior::NoReflection
    }
}

impl Default for EncodeOptions {
    fn default() -> EncodeOptions {
        EncodeOptions::new()
    }
}

pub struct EmitState {
    referent_map: HashMap<RbxId, u32>,
    next_referent: u32,
    options: EncodeOptions,
}

impl EmitState {
    pub fn new(options: EncodeOptions) -> EmitState {
        EmitState {
            referent_map: HashMap::new(),
            next_referent: 0,
            options,
        }
    }

    pub fn map_id(&mut self, id: RbxId) -> u32 {
        match self.referent_map.get(&id) {
            Some(&value) => value,
            None => {
                let referent = self.next_referent;
                self.referent_map.insert(id, referent);
                self.next_referent += 1;
                referent
            }
        }
    }
}

fn serialize_value<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    xml_name: &str,
    value: &RbxValue,
) -> Result<(), NewEncodeError> {
    // Refs need additional state that we don't want to thread through
    // `write_value_xml`, so we handle it here.
    match value {
        RbxValue::Ref { value: id } => write_ref(writer, xml_name, id, state).map_err(Into::into),
        _ => write_value_xml(writer, xml_name, value)
    }
}

fn serialize_instance<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    tree: &RbxTree,
    id: RbxId,
) -> Result<(), NewEncodeError> {
    let instance = tree.get_instance(id).unwrap();
    let mapped_id = state.map_id(id);

    writer.write(XmlWriteEvent::start_element("Item")
        .attr("class", &instance.class_name)
        .attr("referent", &mapped_id.to_string()))?;

    writer.write(XmlWriteEvent::start_element("Properties"))?;

    serialize_value(writer, state, "Name", &RbxValue::String {
        value: instance.name.clone(),
    })?;

    for (property_name, value) in &instance.properties {
        let maybe_serialized_descriptor = if state.options.use_reflection() {
            find_serialized_property_descriptor(&instance.class_name, property_name)
        } else {
            None
        };

        if let Some(serialized_descriptor) = maybe_serialized_descriptor {
            let value_type = match serialized_descriptor.property_type() {
                RbxPropertyTypeDescriptor::Data(value_type) => *value_type,
                RbxPropertyTypeDescriptor::Enum(_enum_name) => RbxValueType::Enum,
                RbxPropertyTypeDescriptor::UnimplementedType(_) => {
                    // Properties with types that aren't implemented yet are
                    // effectively unknown properties, so we handle them
                    // similarly.
                    match state.options.property_behavior {
                        EncodePropertyBehavior::IgnoreUnknown => {
                            continue;
                        }
                        EncodePropertyBehavior::WriteUnknown => {
                            // This conversion will be returned into a no-op by
                            // try_convert_ref
                            value.get_type()
                        }
                        EncodePropertyBehavior::ErrorOnUnknown => {
                            return Err(writer.error(EncodeErrorKind::UnknownProperty {
                                class_name: instance.class_name.clone(),
                                property_name: property_name.clone(),
                            }));
                        }
                        EncodePropertyBehavior::NoReflection | EncodePropertyBehavior::__Nonexhaustive => {
                            unreachable!();
                        }
                    }
                }
            };

            let converted_value = match value.try_convert_ref(value_type) {
                RbxValueConversion::Converted(converted) => Cow::Owned(converted),
                RbxValueConversion::Unnecessary => Cow::Borrowed(value),
                RbxValueConversion::Failed => return Err(writer.error(EncodeErrorKind::UnsupportedPropertyConversion {
                    class_name: instance.class_name.clone(),
                    property_name: property_name.to_string(),
                    expected_type: value_type,
                    actual_type: value.get_type(),
                })),
            };

            serialize_value(writer, state, &serialized_descriptor.name(), &converted_value)?;
        } else {
            match state.options.property_behavior {
                EncodePropertyBehavior::IgnoreUnknown => {}
                EncodePropertyBehavior::WriteUnknown | EncodePropertyBehavior::NoReflection => {
                    // We'll take this value as-is with no conversions on
                    // either the name or value.

                    serialize_value(writer, state, property_name, value)?;
                }
                EncodePropertyBehavior::ErrorOnUnknown => {
                    return Err(writer.error(EncodeErrorKind::UnknownProperty {
                        class_name: instance.class_name.clone(),
                        property_name: property_name.clone(),
                    }));
                }
                EncodePropertyBehavior::__Nonexhaustive => unreachable!()
            }
        }
    }

    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, state, tree, *child_id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}