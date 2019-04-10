use std::collections::HashMap;

use rbx_dom_weak::RbxValue;

use crate::{
    api_dump::Dump,
    reflection_types::RbxInstanceClass,
};

pub struct ReflectionDatabase {
    pub dump: Dump,
    pub default_properties: HashMap<String, HashMap<String, RbxValue>>,
    pub studio_version: [u32; 4],

    pub classes: HashMap<String, RbxInstanceClass>,
}