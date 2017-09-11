use std::collections::HashMap;
use std::io;

pub fn run<I: io::Read, O: io::Write, E: io::Write>(
    argv: Vec<String>,
    env: HashMap<String, String>,
    mut stdin: I,
    mut stdout: O,
    mut stderr: E,
) {
    writeln!(&mut stdout, "Hello world!");
    let mut stdin_contents = String::new();
    stdin.read_to_string(&mut stdin_contents).unwrap();
    writeln!(&mut stderr, "From stdin: {}", stdin_contents);
}
