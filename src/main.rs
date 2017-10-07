extern crate nix;
extern crate users;

use nix::sched;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let user_program = &argv[1];
    let user_argv = &argv[2..];

    let (uid, gid) = current_id();
    unshare_namespaces().expect("unshare");
    write_id_maps(uid, gid).expect("write ID maps");

    let exit_status = run_user_process(&user_program, &user_argv).expect("run user process");
    match exit_status.code() {
        Some(code) => process::exit(code),
        None => panic!("failed to get exit code"),
    }
}

fn run_user_process(user_program: &str, user_argv: &[String]) -> io::Result<process::ExitStatus> {
    let mut user_process = process::Command::new(user_program).args(user_argv).spawn()?;
    user_process.wait()
}

fn unshare_namespaces() -> nix::Result<()> {
    let mut clone_flags = sched::CloneFlags::empty();
    clone_flags.insert(sched::CLONE_NEWUSER);
    clone_flags.insert(sched::CLONE_NEWUTS);
    sched::unshare(clone_flags)
}

fn write_id_maps(uid: u32, gid: u32) -> Result<(), io::Error> {
    overwrite_file("/proc/self/setgroups", b"deny")?;
    write_id_map("g", gid)?;
    write_id_map("u", uid)
}

fn current_id() -> (u32, u32) {
    (users::get_current_uid(), users::get_current_gid())
}

fn write_id_map(kind: &str, id: u32) -> Result<(), io::Error> {
    overwrite_file(
        format!("/proc/self/{}id_map", kind).as_str(),
        format!("0 {} 1", id).as_bytes(),
    )
}

fn overwrite_file(path: &str, contents: &[u8]) -> Result<(), io::Error> {
    let mut f = File::create(path)?;
    f.write(contents)?;
    Ok(())
}
