use crate::types::value::Value;
use ndarray::Array;
use rustler::types::atom::Atom;
use rustler::NifStruct;
use std::collections::HashMap;
#[derive(NifStruct)]
#[module = "NetCDF.Variable"]
pub struct NetCDFVariable {
    pub name: String,
    pub value: Value,
    pub r#type: Atom,
    pub attributes: Vec<(String, Value)>,
    pub shape: Option<HashMap<String, i32>>,
    pub enum_variants: Option<HashMap<String, i32>>,
}

impl NetCDFVariable {
    pub fn new(
        name: String,
        value: Value,
        r#type: Atom,
        attributes: Vec<(String, Value)>,
        shape: Option<HashMap<String, i32>>,
        enum_variants: Option<HashMap<String, i32>>,
    ) -> Self {
        Self {
            name,
            value,
            r#type,
            attributes,
            shape,
            enum_variants,
        }
    }
}
