use std::{collections::HashMap, i64, u64};

const MAGIC_NUMBER: [u8; 4] = [0x67, 0x72, 0x6E, 0x64];

#[derive(Debug)]
pub enum GEF_Type {
    Executable,
    DynamicLib,
    StaticLib
}

#[derive(Debug)]
pub struct Map {
    function_table: HashMap<usize, ([u8;7], [u8;1], [u8; 8])>,
    data_table: HashMap<usize, ([u8; 3], [u8; 1], [u8; 8])>
}

pub fn load_gef(gef_file_data: Vec<u8>) -> Option<((u8, u8, u8), GEF_Type, usize)>{
    
    if gef_file_data.get(0..4) != MAGIC_NUMBER.get(0..4) {
        todo!();
        return None;
    }

    let version = (gef_file_data[5], gef_file_data[6], gef_file_data[7]);
    print!("version: {:?}", version);

    let gef_type: GEF_Type = match gef_file_data[7] {
        1=>GEF_Type::Executable,
        2=>GEF_Type::StaticLib,
        3=>GEF_Type::DynamicLib,
        _=>return None
    };

    let entrypoint = gef_file_data.as_integer(8, 15);

    println!("entrypoint: {}", entrypoint);
    
    let map_size = gef_file_data.as_integer(16, 19);
    println!("map size: {}",map_size);

    let function_table_size = gef_file_data.as_integer(20, 23);
    println!("func table size:{}", function_table_size);
    let data_table_size = gef_file_data.as_integer(24, 27);

    let mut map = Map{function_table: HashMap::new(), data_table: HashMap::new()};
    
    let counter = 0;
    for chunk in gef_file_data[28..function_table_size+28].chunks(16) {
        println!("chunk: {:?}", chunk);
        map.function_table.insert(
            counter,
            (
                chunk[0..=7].try_into().unwrap(),
                chunk[8..=8].try_into().unwrap(),
                chunk[9..=16].try_into().unwrap()
            )
        );
    }
    println!("{:?}", map);
    Some((version, gef_type, entrypoint))
}

trait ByteString { //バイト列（文字列ではない）
    fn as_integer(&self, start: usize, end: usize) -> usize;
}

impl ByteString for Vec<u8> {
    fn as_integer(&self, start: usize, end: usize) -> usize {
        self[start..=end].iter().fold(0, |acc, &n| acc<<8 | n as usize)
    }
}