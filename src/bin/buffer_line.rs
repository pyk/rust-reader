use std::env;
use std::process;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // Parse the arguments
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("Usage: {} file_name.txt", arguments[0]);
        process::exit(1);
    }

    // Access file
    let file_name: String = arguments[1].clone();
    let res_file: Result<File, io::Error> = File::open(file_name);
    let f: File = match res_file {
        Ok(file) => file,
        Err(err) => {
            println!("Err: opening a file: {}", err);
            process::exit(1);
        },
    };

    // read file line by line
    let reader: BufReader<File> = BufReader::new(f);
    for res_line in reader.lines() {
        match res_line {
            Ok(s) => println!("Line: {:?}", s),
            Err(err) => {
                println!("Err: line-by-line: {:?}", err);
                process::exit(1);
            },
        }
    }
}
