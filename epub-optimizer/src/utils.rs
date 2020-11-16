use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::process::Command;

pub fn unzip(path: &str) -> tempfile::TempDir {
    println!("Reading ZIP...");
    let file = File::open(&path).unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();

    println!("Extracting ZIP...");
    let tmp = tempfile::tempdir().unwrap();
    for i in 0..zip.len() {
        let mut input = zip.by_index(i).unwrap();
        if input.name().ends_with("/") {
            continue;
        }
        let input_path = input.sanitized_name();

        let output_path = tmp.path().join(input_path);
        let _ = fs::create_dir_all(output_path.parent().unwrap());
        let mut output = File::create(output_path).unwrap();

        io::copy(&mut input, &mut output).unwrap();
    }

    tmp
}

pub fn gen_epub(path: &str, tmp: &tempfile::TempDir) {
    println!("Zipping...");
    let wd = env::current_dir().unwrap();
    let path_abs = fs::canonicalize(&path).unwrap();

    let _ = fs::remove_file(&path);
    env::set_current_dir(&tmp).unwrap();
    let mut cmd = Command::new("zip");
    cmd.arg("-9r");
    cmd.arg(&path_abs);
    for path in fs::read_dir(".").unwrap() {
        cmd.arg(path.unwrap().path());
    }
    cmd.output().unwrap();
    env::set_current_dir(wd).unwrap();
}
