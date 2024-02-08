use std::collections::HashMap;

use nom::{
    bytes::{complete::{tag, take}},
    IResult, number::complete::{be_u8, be_u64, be_u32}
};

const MAGIC_NUMBER: [u8; 4] = [0x67, 0x72, 0x6E, 0x64];

#[derive(Debug)]
pub enum GEFType {
    Executable,
    DynamicLib,
    StaticLib
}

#[derive(Debug)]
pub struct GEFHeader {
    magic_number: [u8; 4],
    version: (u8, u8, u8),
    gef_file_type: GEFType,
    entrypoint: u64,
    map_size: u32
}

#[derive(Debug)]
pub struct Map {
    function_table: HashMap<usize, ([u8;7], [u8;1], [u8; 8])>,
    data_table: HashMap<usize, ([u8; 3], [u8; 1], [u8; 8])>
}

pub struct MapField {
    name: [u8; 7],
    is_private: u8,
    address: u32
}

pub fn load_gef_header(gef_file_data: &[u8]) ->IResult<&[u8], GEFHeader> {
    // Parsing magic number
    let (input, magic_number) = tag(&MAGIC_NUMBER)(gef_file_data)?;
    let magic_number = magic_number.try_into().unwrap();

    // parsing version
    let (input, version) = take(3u8)(input)?;

    // parsing GEF file type
    let (input, gef_file_type) = be_u8(input)?;
    let gef_file_type = match gef_file_type {
        0=>GEFType::Executable,
        1=>GEFType::DynamicLib,
        2=>GEFType::StaticLib,
        _=>panic!("Failed to loading file because of bad GEF file type")
    };

    // Parsing entrypoint
    let (input, entrypoint) = be_u64(input)?;

    // Parsing map_size
    let (input, map_size) = be_u32(input)?;
    Ok((
        input,
        GEFHeader {
            magic_number,
            version: (version[0], version[1], version[2]),
            gef_file_type,
            entrypoint,
            map_size
        }
    ))
}

pub fn create_map(input: &[u8]) { // "input" must be without headers.
    let mut map = Map{
        function_table: HashMap::new(), 
        data_table: HashMap::new()
    };


}

pub fn load_gef(gef_file_data: &[u8]) -> (&[u8], GEFHeader) {
    let (without_header, header) = load_gef_header(gef_file_data).unwrap();
    return (without_header, header);
}