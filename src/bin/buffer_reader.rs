use std::env;
use std::process;
use std::io::BufReader;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // Parse the arguments
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("Usage: {} file_name.txt", arguments[0]);
        process::exit(1);
    }

    let file_name: String = arguments[1].clone();
    let res_file: Result<File, io::Error> = File::open(file_name);
    let f: File = match res_file {
        Ok(file) => file,
        Err(err) => {
            println!("Err: opening file {:?}", err);
            process::exit(1);
        },
    };

    let mut reader: BufReader<File> = BufReader::new(f);
    let mut test: String = String::new();
    let res_read: Result<usize, io::Error>;
    res_read = reader.read_to_string(&mut test);
    match res_read {
        Ok(n) => println!("{}: {:?}", n, test),
        Err(err) => {
            println!("Err: reading a file: {:?}", err);
        },
    }
}
