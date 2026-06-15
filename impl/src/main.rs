//! Vyro CLI — the reference toolchain for VyroLang.
//!
//! Usage:
//!   vyro run <file.vy>     compile and execute a program
//!   vyro check <file.vy>   parse + compile only (no execution)
//!   vyro version

use std::process::exit;

use vyro::{compile_source, run_source};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        exit(64);
    }
    match args[1].as_str() {
        "version" | "--version" | "-v" => {
            println!("Vyro {}", VERSION);
        }
        "run" => {
            let path = arg_or_die(&args, 2, "run");
            let src = read_or_die(&path);
            if let Err(e) = run_source(&src) {
                eprintln!("{}", e);
                exit(70);
            }
        }
        "check" => {
            let path = arg_or_die(&args, 2, "check");
            let src = read_or_die(&path);
            match compile_source(&src) {
                Ok(_) => println!("ok: {} compiles cleanly", path),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }
        }
        other => {
            eprintln!("unknown command: {}", other);
            usage();
            exit(64);
        }
    }
}

fn usage() {
    eprintln!("Vyro {} — the VyroLang toolchain", VERSION);
    eprintln!("usage:");
    eprintln!("  vyro run <file.vy>     compile and execute");
    eprintln!("  vyro check <file.vy>   parse + compile only");
    eprintln!("  vyro version");
}

fn arg_or_die(args: &[String], i: usize, cmd: &str) -> String {
    match args.get(i) {
        Some(s) => s.clone(),
        None => {
            eprintln!("error: '{}' needs a file path", cmd);
            exit(64);
        }
    }
}

fn read_or_die(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read '{}': {}", path, e);
            exit(66);
        }
    }
}
