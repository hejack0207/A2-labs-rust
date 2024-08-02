use std::borrow::Borrow;
use std::env;
use std::ffi::OsStr;
#[cfg(feature = "cmake-build")]
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

fn main() {
    eprintln!("Building and linking ue statically");
    build_ue();
}

#[cfg(not(feature = "cmake-build"))]
fn build_ue() {
    let mut cflags = Vec::new();
    if let Ok(var) = env::var("CFLAGS") {
        cflags.push(var);
    }

    let mut ldflags = Vec::new();
    if let Ok(var) = env::var("LDFLAGS") {
        ldflags.push(var);
    }

    env::set_var("CFLAGS", cflags.join(" "));
    env::set_var("LDFLAGS", ldflags.join(" "));

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR missing");

    if !Path::new(&out_dir).join("LICENSE").exists() {
        println!("Copying ue");
        run_command_or_fail(".", "cp", &["-aL", "ue/.", &out_dir]);
    }

    println!("Compiling ue");
    if let Some(makeflags) = env::var_os("CARGO_MAKEFLAGS") {
        env::set_var("MAKEFLAGS", makeflags);
    }
    run_command_or_fail(
        &out_dir,
        "make",
        &["libs"],
    );

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ue");
    // println!("cargo:rustc-link-lib=dylib=ue");
    println!("cargo:root={}", out_dir);
}
