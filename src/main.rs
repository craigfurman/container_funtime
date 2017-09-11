extern crate container_funtime;

use std::env;
use std::io;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let vars = env::vars().collect();
    container_funtime::run(argv, vars, io::stdout());
}
