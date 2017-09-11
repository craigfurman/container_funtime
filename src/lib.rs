use std::collections::HashMap;
use std::io;

pub fn run<O: io::Write>(argv: Vec<String>, env: HashMap<String, String>, mut stdout: O) {
    writeln!(&mut stdout, "Hello world!");
}
