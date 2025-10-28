use netcdf::File;
use rustler::{NifStruct, ResourceArc};

pub struct NetCDFFileRef(pub File);

#[derive(NifStruct)]
#[module = "NetCDF.File"]
pub struct NetCDFFile {
    pub file: ResourceArc<NetCDFFileRef>,
    pub filename: String,
    pub variables: Vec<String>,
}

impl NetCDFFile {
    pub fn new(file: ResourceArc<NetCDFFileRef>, filename: &str, variables: Vec<String>) -> Self {
        Self {
            file: file,
            filename: filename.to_string(),
            variables,
        }
    }
}
