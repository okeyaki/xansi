extern crate xansi;

use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
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
