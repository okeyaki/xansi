extern crate clap;
extern crate xansi;

use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use clap::App;

fn main() {
    let _ = App::new("xansi")
        .about("A CLI tool to remove ANSI escape sequences")
        .version("0.2.0")
        .get_matches();

    let mut stream = BufReader::new(stdin());
    loop {
        let mut line = String::new();
        match stream.read_line(&mut line) {
            Ok(0) => {
                // Reached EOF.
                break;
            }
            Ok(_) => {
                print!("{}", xansi::remove_ansi_escape_sequences(&line));
            }
            Err(error) => {
                println!("{}", error);

                break;
            }
        }
    }
}
