extern crate container_funtime;

use std::collections::HashMap;
use std::str;

#[test]
fn it_says_hello() {
    let mut stdout: Vec<u8> = Vec::new();
    container_funtime::run(vec![], HashMap::new(), &mut stdout);
    let final_stdout = str::from_utf8(&stdout).unwrap();
    assert_eq!("Hello world!\n", final_stdout);
}
