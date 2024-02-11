use std::{collections::HashMap};

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
    function_table: HashMap<usize, MapField>,
    data_table: HashMap<usize, MapField>
}

#[derive(Debug)]
pub struct MapField {
    name_address: u32,
    name_size: u64,
    address: u64
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

pub fn create_map(input: &[u8], map_size: u32) -> IResult<&[u8], Map> { // "input" must be without headers.

    let map_field_size = std::mem::size_of::<MapField> as u32;

    let mut map = Map {
        function_table: HashMap::new(), 
        data_table: HashMap::new()
    };
    let input = &input[..map_size as usize]; // At this point, input became raw map binary
    
    let (input, func_table_size) = be_u32(input)?;
    let (input, data_table_size) = be_u32(input)?;

    //let mut func_index = 0;
    //let mut data_index = 0;
    
    if map_size != 4+4+func_table_size+data_table_size {
        panic!("Error: The size or composition of the map is invalid.")
    };

    let num_of_func = func_table_size / map_field_size;
    let num_of_data = data_table_size / map_field_size;

    for (i, chunk) in input.chunks(map_field_size as usize).enumerate() {
        let (chunk, name_address) = be_u32(chunk)?;
        let (chunk, name_size) = be_u64(chunk)?;
        let (_, address) = be_u64(chunk)?;

        let map_field = MapField{name_address, name_size, address};

        if i <= (num_of_func as usize -1) {
            map.function_table.insert(i, map_field).unwrap();
        } else {
            map.data_table.insert(i - (num_of_func as usize - 1), map_field).unwrap();
        }
    };
    Ok((input, map))
}

pub fn load_gef(gef_file_data: &[u8]) -> (&[u8], GEFHeader) {
    let (without_header, header) = load_gef_header(gef_file_data).unwrap();
    let (without_map, map) = create_map(without_header, header.map_size).unwrap();
    return (without_header, header);
}