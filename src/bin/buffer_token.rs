use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {:?} file_name.txt", args[0]);
        process::exit(1);
    }

    // Access file
    let file_name: String = args[1].clone();
    let f: File = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            println!("Err: opening file: {:?}", err);
            process::exit(1);
        },
    };

    // Read file word by word
    let reader: BufReader<File> = BufReader::new(f);
    let mut bytes: Vec<u8> = Vec::with_capacity(256);
    for res_byte in reader.bytes() {
        let b: u8 = match res_byte {
            Ok(b) => b,
            Err(err) => {
                println!("Err: read bytes: {:?}", err);
                process::exit(1);
            },
        };
        let c: char = b as char;
        if !c.is_whitespace() {
            if bytes.len() == 256 { bytes.reserve(256); };
            bytes.push(b);
        } else {
            // skip if bytes is empty
            if bytes.len() == 0 { continue; };
            let word: String = match String::from_utf8(bytes.clone()) {
                Ok(s) => s,
                Err(err) => {
                    println!("Err: convert bytes to utf-8: {:?}", err);
                    println!("bytes: {:?}", bytes);
                    continue;
                },
            };

            println!("token: {:?}", word);
            bytes.clear();
        }
    }
}
