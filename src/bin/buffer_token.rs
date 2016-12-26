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

    // Extract token that separated by whitespace from file
    let reader: BufReader<File> = BufReader::new(f);
    const MAX_BYTES: usize = 256;
    let mut bytes: Vec<u8> = Vec::with_capacity(MAX_BYTES);
    for res_byte in reader.bytes() {
        let byte: u8 = match res_byte {
            Ok(b) => b,
            Err(err) => {
                println!("Err: read bytes: {:?}", err);
                process::exit(1);
            },
        };

        // UTF-8 range
        // 0 - 127 => single-byte character
        // 128 - 191 => Continuation bytes
        // 194 - 244 => leading bytes
        // 192,193,245-255 => invalid utf-8
        match byte {
            // Handle single-byte character
            byte if (byte <= 127) => {
                let c: char = byte as char;
                if !c.is_whitespace() {
                    if bytes.len() == MAX_BYTES { 
                        bytes.reserve(MAX_BYTES); 
                    };
                    bytes.push(byte);
                } else {
                    // skip if bytes is empty
                    if bytes.len() == 0 { continue; };
                    let token: String;
                    token = match String::from_utf8(bytes.clone()) {
                        Ok(s) => s,
                        Err(err) => {
                            println!("Err: convert bytes to String: {:?}", 
                                err);
                            bytes.clear();
                            continue;
                        },
                    };

                    println!("token: {:?}", token);
                    bytes.clear();
                }
            }
            // Just push the leading and continuation bytes to bytes buffer
            _ => {
                bytes.push(byte);
            }
        }
    }
}
