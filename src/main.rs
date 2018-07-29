use std::env;
use std::fs::File;
use std::io::prelude::*;

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
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{}", contents);
}
