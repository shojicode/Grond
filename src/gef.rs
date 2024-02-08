use std::{env, error::Error, fs::{File}, io::Read};
mod loader;
mod runtime;

const VERSION: &str = "
    Grond Runtime(GRT)

      Version 0.0.1
";

const HELP: &str = "
    Grond Runtime Help
    
# Command Format
$ grt [options] file_path [args]

# Options
--version / -v : Show GRT version.
--help    / -h : Show GRT help.
";

fn main() {
    // 0. Parse Commandline Args
    let args: Vec<String> = env::args().collect();
    let args = args.iter().map(|arg| arg.as_str());
    
    let mut gef_file_path: Option<&str> = None;
    let mut gef_cmd_args: Vec<&str> = Vec::new();

    let mut counter = 0;
    for i in args {
        counter += 1;
        match i {
            "--version" | "-v" =>{
                    println!("{}", VERSION);
                    return;
                },
            "--help" | "-h" =>{
                    println!("{}", HELP);
                    return;
                },
            "--native-path" | "-np" => todo!(),
            _ => {
                if counter == 1 {continue;}
                match gef_file_path {
                    None => gef_file_path = Some(i),
                    _ => gef_cmd_args.push(i)
                }
            }
        }
    }

    // 1. Loading GEF Files
    let mut gef_file = match File::open(gef_file_path.unwrap()) {
        Ok(file) => file,
        Err(why) => panic!("Coudn't read that file: {}", why)   
    };
    let mut gef_file_data = Vec::new();
    let _ = gef_file.read_to_end(&mut gef_file_data);
    let gef_file_data = gef_file_data.as_slice();
    println!("{:?}", loader::load_gef(gef_file_data));

    ()
}