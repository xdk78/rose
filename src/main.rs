use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod result;
mod scanner;
mod token;

use result::Result;

use scanner::TokenIterator;

fn main() {
    use result::Error::*;
    let args: Vec<String> = env::args().collect();

    let res: Result<()> = match args.len() {
        2 => {
            let file = &args[1];
            run_file(file)
        }
        _ => Err(Box::new(Usage)),
    };

    match res {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn run_file(filename: &str) -> Result<()> {
    let mut buf = String::new();
    {
        File::open(filename)?.read_to_string(&mut buf)?;
    }

    run(&buf)
}

fn run(buf: &str) -> Result<()> {
    let mut tokens = buf.chars().tokens();
    while let Some(res) = tokens.next() {
        match res {
            Ok(t) => println!("{}", t),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
