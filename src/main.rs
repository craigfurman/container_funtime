extern crate getopts;
extern crate nix;
extern crate users;

use getopts::Options;
use getopts::ParsingStyle;
use nix::mount;
use nix::sched;
use nix::unistd;
use std::env;
use std::fs;
use std::path;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process;

const REEXEC: &str = "/proc/self/exe";

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config = Config::parse_cmdline(&argv);

    if &argv[0] == REEXEC {
        child(config)
    } else {
        parent(argv)
    }
}

fn parent(argv: Vec<String>) {
    let user = current_user();
    unshare_namespaces(
        &[
            sched::CLONE_NEWUSER,
            sched::CLONE_NEWNS,
            sched::CLONE_NEWPID,
            sched::CLONE_NEWUTS,
        ],
    ).expect("parent unshare ns");
    write_id_maps(&user).expect("write ID maps");

    let child_exit_status = process::Command::new(REEXEC)
        .args(&argv[1..])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    match child_exit_status.code() {
        Some(code) => process::exit(code),
        None => panic!("failed to get exit code"),
    }
}

fn child(config: Config) {
    mount::mount::<str, str, str, str>(
        Some("proc"),
        "/proc",
        Some("proc"),
        mount::MsFlags::empty(),
        None,
    ).expect("remounting /proc");

    let rootfs = &config.rootfs;

    mount::mount::<str, str, str, str>(None, "/", None, mount::MS_PRIVATE | mount::MS_REC, None)
        .unwrap();

    mount::mount::<str, str, str, str>(Some(rootfs), &config.rootfs, None, mount::MS_BIND, None)
        .unwrap();

    let rootfs_path = path::Path::new(&config.rootfs);
    let oldrootfs = "oldrootfs";
    let old_rootfs_path = rootfs_path.join(oldrootfs);
    fs::create_dir_all(&old_rootfs_path).unwrap();

    mount::mount::<str, str, str, str>(
        Some("/proc"),
        rootfs_path.join("proc").to_str().unwrap(),
        Some("proc"),
        mount::MS_BIND,
        None,
    ).expect("bind mounting /proc");

    unistd::pivot_root(&rootfs_path.to_path_buf(), &old_rootfs_path).expect("pivot_root");
    env::set_current_dir("/").expect("chdir /");

    mount::umount2(oldrootfs, mount::MNT_DETACH).expect("unmount old rootfs");
    fs::remove_dir(oldrootfs).expect("deleting old rootfs mountpoint");

    let exit_status = run_user_process(&config.user_program, &config.user_argv)
        .expect("run user process");
    match exit_status.code() {
        Some(code) => process::exit(code),
        None => panic!("failed to get exit code"),
    }
}

struct Config {
    rootfs: String,
    user_program: String,
    user_argv: Vec<String>,
}

impl Config {
    fn parse_cmdline(argv: &[String]) -> Config {
        let mut opts = Options::new();
        opts.reqopt(
            "r",
            "rootfs",
            "root filesystem to pivot_root into",
            "/path/to/some/rootfs",
        );
        opts.parsing_style(ParsingStyle::StopAtFirstFree);
        let parsed_cmdline = opts.parse(&argv[1..]).expect("parse command line");
        let rootfs = parsed_cmdline.opt_str("r").expect("get rootfs flag value");
        let user_program = parsed_cmdline.free[0].clone();
        let user_argv = parsed_cmdline.free[1..].to_vec();
        Config {
            rootfs,
            user_program,
            user_argv,
        }
    }
}

fn run_user_process(user_program: &str, user_argv: &[String]) -> io::Result<process::ExitStatus> {
    let mut user_process = process::Command::new(user_program).args(user_argv).spawn()?;
    user_process.wait()
}

fn unshare_namespaces(ns: &[sched::CloneFlags]) -> nix::Result<()> {
    let mut clone_flags = sched::CloneFlags::empty();
    for clone_flag in ns {
        clone_flags.insert(*clone_flag);
    }
    sched::unshare(clone_flags)
}

fn write_id_maps(user: &users::User) -> Result<(), io::Error> {
    overwrite_file("/proc/self/setgroups", b"deny")?;
    write_id_map("g", user.primary_group_id())?;
    write_id_map("u", user.uid())
}

fn current_user() -> users::User {
    users::get_user_by_uid(users::get_current_uid()).unwrap()
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
