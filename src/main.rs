use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let user_program = &argv[1];
    let user_argv = &argv[2..];

    let mut user_process = process::Command::new(user_program)
        .args(user_argv)
        .spawn()
        .expect("failed to start user process");
    let exit_status = user_process.wait().expect("failed to wait on user process");
    match exit_status.code() {
        Some(code) => process::exit(code),
        None => panic!("failed to get exit code"),
    }
}
