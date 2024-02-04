const MAGIC_NUMBER: [u8; 4] = [0x67, 0x72, 0x6E, 0x64];

#[derive(Debug)]
enum GEF_Type {
    Executable,
    DynamicLib,
    StaticLib
}

pub fn load_gef(gef_file_data: Vec<u8>) -> Option<()>{
    let counter = 0;
    
    if gef_file_data.get(0..4) != MAGIC_NUMBER.get(0..4) {
        todo!();
        return None;
    }

    let version = (gef_file_data[5], gef_file_data[6], gef_file_data[7]);
    print!("{:?}", version);

    let gef_type: GEF_Type = match gef_file_data[7] {
        1=>GEF_Type::Executable,
        2=>GEF_Type::StaticLib,
        3=>GEF_Type::DynamicLib,
        _=>return None
    };

    println!("{:?}", gef_file_data[15]);

    let entrypoint: u64 = (&gef_file_data[8..16]).iter()
            .fold(0, |acc, &n| acc<<8 | n as u64);

    Some(())
}