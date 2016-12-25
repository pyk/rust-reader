use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} file_name.txt", args[0]);
        process::exit(1);
    }

    // Access file
    let file_name: String = args[1].clone();
    let f: File = match File::open(file_name) {
        Ok(f) => f,
        Err(err) => {
            println!("Err: open file: {:?}", err);
            process::exit(1);
        },
    };

    // Iterate over bytes
    let reader: BufReader<File> = BufReader::new(f);
    for res_byte in reader.bytes() {
        let byte: u8 = match res_byte {
            Ok(b) => b,
            Err(err) => {
                println!("Err: read byte: {:?}", err);
                process::exit(1);
            }
        };

        println!("byte: {:?}", byte);
    }
}
