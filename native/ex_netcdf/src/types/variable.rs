use crate::types::value::Value;
use ndarray::Array;
use rustler::types::atom::Atom;
use rustler::NifStruct;
#[derive(NifStruct)]
#[module = "NetCDF.Variable"]
pub struct NetCDFVariable {
    pub name: String,
    pub value: Value,
    pub r#type: Atom,
    pub attributes: Vec<(String, Value)>,
    pub shape: Option<Vec<usize>>,
}

impl NetCDFVariable {
    pub fn new(name: String, value: Value, r#type: Atom, attributes: Vec<(String, Value)>) -> Self {
        Self {
            name,
            value,
            r#type,
            attributes,
        }
    }
}
