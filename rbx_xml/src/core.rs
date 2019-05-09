use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;
use rbx_reflection::RbxPropertyDescriptor;

use crate::{
    deserializer_core::XmlEventReader,
    serializer_core::XmlEventWriter,
    error::{DecodeError, EncodeError},
};

pub trait XmlType<T: ?Sized> {
    const XML_TAG_NAME: &'static str;

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &T,
    ) -> Result<(), EncodeError>;

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError>;
}

pub fn find_canonical_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static RbxPropertyDescriptor> {
    let class_descriptor = rbx_reflection::get_class_descriptor(class_name)?;

    let mut current_class_descriptor = class_descriptor;

    // We need to find the canonical property descriptor associated with
    // the property we're trying to deserialize.
    //
    // At each step of the loop, we're checking a new class descriptor
    // to see if it has an entry for the property name we're looking for.
    loop {
        // If this class descriptor knows about this property name,
        // we're pretty much done!
        if let Some(property_descriptor) = current_class_descriptor.get_property_descriptor(property_name) {
            if property_descriptor.is_canonical() {
                // The property name in the XML was the canonical name
                // and also the serialized name, hooray!

                return Some(property_descriptor);
            }

            if let Some(canonical_name) = property_descriptor.canonical_name() {
                // This property has a canonical form that we'll map
                // from the XML name.

                return current_class_descriptor.get_property_descriptor(canonical_name);
            } else {
                // This property doesn't have a canonical form, we we'll
                // skip serializing it by declaring there isn't a
                // canonical property descriptor for it.

                return None;
            }
        }

        if let Some(superclass_name) = current_class_descriptor.superclass() {
            // If a property descriptor isn't found in our class, check
            // our superclass.

            current_class_descriptor = rbx_reflection::get_class_descriptor(superclass_name)
                .expect("Superclass in rbx_reflection didn't exist");
        } else {
            // This property isn't known by any class in the reflection
            // database.

            return None;
        }
    }
}