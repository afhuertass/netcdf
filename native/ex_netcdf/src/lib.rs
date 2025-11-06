use netcdf::Attribute;
use netcdf::{Extents, Extents::All};
use rustler::ResourceArc;
use rustler::{Env, Term};
mod error;
mod types;

use error::NetCDFError;
use types::file::{NetCDFFile, NetCDFFileRef};
use types::value::Value;
use types::variable::NetCDFVariable;

use std::collections::HashMap;
rustler::atoms! {
    nil,
    ok,
    non_numeric,
    i8_t = "i8",
    i16_t = "i16",
    i32_t = "i32",
    i64_t = "i64",
    u8_t = "u8",
    u16_t = "u16",
    u32_t = "u32",
    u64_t = "u64",
    f32_t = "f32",
    f64_t = "f64",
    string_t = "string"
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(NetCDFFileRef, env);
    true
}

#[rustler::nif]
fn file_open(filename: &str) -> Result<NetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = netcdf::open(filepath)?;
    let file_arc = ResourceArc::new(NetCDFFileRef(file));
    Ok(NetCDFFile::new(file_arc, filename, Vec::<String>::new()))
}

#[rustler::nif]
fn file_variables(ex_file: NetCDFFile) -> Result<Vec<String>, NetCDFError> {
    let file: &netcdf::File = &ex_file.file.0;
    let result = file.variables().map(|var| var.name()).collect();
    Ok(result)
}

#[rustler::nif]
fn file_open_with_variables(filename: &str) -> Result<NetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = netcdf::open(filepath)?;
    let variables = file.variables().map(|var| var.name()).collect();

    let file_arc = ResourceArc::new(NetCDFFileRef(file));
    Ok(NetCDFFile::new(file_arc, filename, variables))
}

#[rustler::nif]
fn variable_values(
    ex_file: NetCDFFile,
    variable_name: &str,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let file: &netcdf::File = &ex_file.file.0;

    file.variable(variable_name)
        .ok_or(NetCDFError::NotFound())
        .and_then(|var| get_variable_values(&var))
}

fn get_variable_values(
    variable: &netcdf::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    match variable.vartype() {
        netcdf::types::NcVariableType::Float(_) => load_numeric_variable_values::<u16>(variable),
        netcdf::types::NcVariableType::Int(_) => load_numeric_variable_values::<u16>(variable),
        netcdf::types::NcVariableType::Compound(_) => load_numeric_variable_values::<u16>(variable),
        netcdf::types::NcVariableType::Opaque(_) => load_numeric_variable_values::<u16>(variable),
        netcdf::types::NcVariableType::String => load_string_variable_values(variable),
        netcdf::types::NcVariableType::Vlen(_) => load_string_variable_values(variable),
        netcdf::types::NcVariableType::Enum(e) => {
            load_enum_type_variable_values::<i32>(variable, e)
        }
        _ => load_numeric_variable_values::<u8>(variable),
    }
}

fn _create_var_map(enum_type: netcdf::types::EnumType) -> Result<HashMap<String, i32>, String> {
    let mut variant_map = HashMap::new();
    let names = &enum_type.fieldnames;

    let values_len = match &enum_type.fieldvalues {
        netcdf::types::EnumTypeValues::U8(v) => v.len(),
        netcdf::types::EnumTypeValues::U16(v) => v.len(),
        netcdf::types::EnumTypeValues::U32(v) => v.len(),
        netcdf::types::EnumTypeValues::U64(v) => v.len(),
        netcdf::types::EnumTypeValues::I8(v) => v.len(),
        netcdf::types::EnumTypeValues::I16(v) => v.len(),
        netcdf::types::EnumTypeValues::I32(v) => v.len(),
        netcdf::types::EnumTypeValues::I64(v) => v.len(),
    };

    if names.len() != values_len {
        return Err(format!(
            "Mismatched lengths: {} names but {} values for enum {}",
            names.len(),
            values_len,
            enum_type.name
        ));
    }

    // Match on the values to correctly access the inner vector and cast the elements.
    match &enum_type.fieldvalues {
        // Zipping names with values and inserting into the map, casting to i32.
        // NOTE: For U64 and I64, we rely on the values fitting within i32.
        // For production code, you would add a check here for potential overflow/underflow.
        netcdf::types::EnumTypeValues::U8(values) => {
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::U16(values) => {
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::U32(values) => {
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::U64(values) => {
            // Note: U64 values MUST fit in i32 for this to be reliable.
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::I8(values) => {
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::I16(values) => {
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
        netcdf::types::EnumTypeValues::I32(values) => {
            // Perfect match, no cast needed, but included for completeness.
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value);
            }
        }
        netcdf::types::EnumTypeValues::I64(values) => {
            // Note: I64 values MUST fit in i32 for this to be reliable.
            for (name, &value) in names.iter().zip(values.iter()) {
                variant_map.insert(name.clone(), value as i32);
            }
        }
    }

    Ok(variant_map)
}

fn create_variant_map(
    ncvariabletype: netcdf::types::NcVariableType,
) -> Result<HashMap<String, i32>, String> {
    // A crucial safety check: the number of names must match the number of values.
    match ncvariabletype {
        netcdf::types::NcVariableType::Enum(e) => _create_var_map(e),
        _ => Err("The variable is not enum type".to_string()),
    }
}
fn load_enum_type_variable_values<T>(
    variable: &netcdf::Variable,
    e: netcdf::types::EnumType,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError>
where
    T: netcdf::NcTypeDescriptor + Copy,
    Value: From<Vec<i8>>,
{
    let error = netcdf::Error::Str(format!("unable to load type asdasd {:#?}", e));

    let size = variable.len();
    let type_name = variable.name();
    let values = variable.get_raw_values(netcdf::Extents::All).unwrap();
    //.map_err(|_| error::NetCDFError::NetCDF(error))?;
    Ok((Value::from(values), as_type_atom(&type_name)))
}

fn load_numeric_variable_values<T>(
    variable: &netcdf::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError>
where
    T: netcdf::NcTypeDescriptor + Copy,
    Value: From<Vec<T>>,
{
    print!("{:#?}", variable.vartype());
    let type_name = variable.name();
    let error = netcdf::Error::Str(format!(
        "unable to load type asdasd {:#?}",
        variable.vartype()
    ));

    let value = variable
        .get_values::<T, _>(..)
        .map_err(|_| error::NetCDFError::NetCDF(error))?;
    Ok((Value::from(value), as_type_atom(&type_name)))
}

fn load_enum_type_values(variable: &netcdf::Variable) -> Result<HashMap<String, i32>, String> {
    let vartype = variable.vartype();
    let rr = create_variant_map(vartype);
    rr
}

fn get_variable_dims(variable: &netcdf::Variable) -> Result<HashMap<String, i32>, String> {
    let dims = variable.dimensions();
    let mut resp: HashMap<String, i32> = HashMap::new();
    for (_index, dim) in dims.iter().enumerate() {
        let name = dim.name();
        let dim_len = dim.len();
        resp.insert(name, dim_len as i32);
    }
    Ok(resp)
}
fn load_string_variable_values(
    variable: &netcdf::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let time_len = variable.len() as usize;
    let mut values: Vec<String> = Vec::with_capacity(time_len);

    for i in 0..time_len {
        let error = netcdf::Error::Str("unable to load string variable".to_string());
        let value: String = variable
            .get_string::<Extents>(i.into())
            .map_err(|_| error::NetCDFError::NetCDF(error))?;
        values.push(value);
    }

    Ok((Value::from(values), string_t()))
}

fn as_type_atom(type_name: &str) -> rustler::types::atom::Atom {
    match type_name {
        "i8" => i8_t(),
        "i16" => i16_t(),
        "i32" => i32_t(),
        "i64" => i64_t(),
        "u8" => u8_t(),
        "u16" => u16_t(),
        "u32" => u32_t(),
        "u64" => u64_t(),
        "f32" => f32_t(),
        "f64" => f64_t(),
        _ => non_numeric(),
    }
}

#[rustler::nif]
fn variable_attributes(
    ex_file: NetCDFFile,
    variable_name: &str,
) -> Result<Vec<(String, Value)>, NetCDFError> {
    let file = &ex_file.file.0;
    file.variable(variable_name)
        .ok_or(NetCDFError::NotFound())
        .map(|var| get_variable_attributes(&var))
}

fn get_variable_attributes(variable: &netcdf::Variable) -> Vec<(String, Value)> {
    variable
        .attributes()
        .map(parse_variable_attribute)
        .collect()
}

#[rustler::nif]
fn variable_load(ex_file: NetCDFFile, variable_name: &str) -> Result<NetCDFVariable, NetCDFError> {
    let file: &netcdf::File = &ex_file.file.0;
    let variable = file
        .variable(variable_name)
        .ok_or(NetCDFError::NotFound())?;
    let (values, value_type) = get_variable_values(&variable)?;

    let attributes = get_variable_attributes(&variable);

    let var_enum_type = load_enum_type_values(&variable);
    let okthing = var_enum_type.ok();

    let dim_map = get_variable_dims(&variable);
    let okmap = dim_map.ok();
    Ok(NetCDFVariable::new(
        variable_name.to_string(),
        values,
        value_type,
        attributes,
        okmap,
        okthing,
    ))
}

fn parse_variable_attribute(attr: Attribute) -> (String, Value) {
    let name = attr.name().to_string();
    let value = match attr.value() {
        Err(_) => Value::Atom(nil()),
        Ok(attr_value) => Value::from(attr_value),
    };

    (name, value)
}

rustler::init!(
    "Elixir.NetCDF.Native",
    [
        file_open,
        file_variables,
        file_open_with_variables,
        variable_load,
        variable_values,
        variable_attributes
    ],
    load = on_load
);
