extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

pub fn remove_ansi_escape_sequences(s: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    }

    return RE.replace_all(&s, "").to_string();
}
