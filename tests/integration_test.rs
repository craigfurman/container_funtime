extern crate container_funtime;

use std::collections::HashMap;
use std::str;

#[test]
fn it_says_hello() {
    let mut stdout: Vec<u8> = Vec::new();
    let mut stderr: Vec<u8> = Vec::new();
    let stdin = String::from("I'm stdin");

    container_funtime::run(
        vec![],
        HashMap::new(),
        stdin.as_bytes(),
        &mut stdout,
        &mut stderr,
    );

    let final_stdout = str::from_utf8(&stdout).unwrap();
    let final_stderr = str::from_utf8(&stderr).unwrap();
    assert_eq!("Hello world!\n", final_stdout);
    assert_eq!("From stdin: I'm stdin\n", final_stderr);
}
