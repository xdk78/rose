use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let cmd = &args[1];
            let file = &args[2];
            // parse the command
            match &cmd[..] {
                "run" => run_file(file),
                _ => {
                    eprintln!("error: invalid command");
                }
            }
        }
        _ => {
            eprintln!("error: no arguments passed!");
        }
    }
}

fn run_file(filename: &str) {
    let mut file = BufReader::new(File::open(filename).expect("file not found"));
    let mut buf = Vec::<u8>::new();

    while file.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("Token: {}", c);
        }
        buf = s.into_bytes();
        buf.clear();
    }
}
