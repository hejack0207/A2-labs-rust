use clap::{App, Arg, SubCommand, ArgMatches, Values};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::path::Path;
use walkdir::WalkDir;

use utils::{unzip, gen_epub};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("optimize")
        .about("optimizes and edits metadata of .epub files")
        .arg(
            Arg::with_name("title")
                .help("change epub title")
                .short("t")
                // .required(false)
                // .takes_value(true)
                .long("title"),
        )
}

pub fn action(matches: &ArgMatches, files: Values){
    let optimize = !matches.is_present("no-optimize");
    let metadata = matches.is_present("metadata");

    let mut bytes_saved: i64 = 0;
    for path in files {
        println!("{}:", path);
        let original_len = fs::metadata(path).unwrap().len() as i64;

        process(path, optimize, metadata, matches.value_of("title"));

        let optimized_len = fs::metadata(path).unwrap().len() as i64;
        bytes_saved += original_len - optimized_len;

        println!();
    }

    if optimize {
        println!("{}KiB saved in total.", bytes_saved / 1024);
    } else {
        println!("Done.")
    }
}
