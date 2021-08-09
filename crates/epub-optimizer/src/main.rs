extern crate clap;
extern crate tempfile;
extern crate walkdir;
extern crate xmltree;
extern crate zip;
extern crate epub;

use clap::App;
use clap::Arg;

mod optimize;
mod utils;
mod check;

fn main() {
    let matches = App::new("epub-toolbox")
        .about("A command-line app that check and manipulate .epub files")
        .subcommand(optimize::subcommand())
        .subcommand(check::subcommand())
        .arg(
            Arg::with_name("files")
                .help("List of files to process")
                .required(true)
                .min_values(1),
        ).get_matches();

    let files = matches.values_of("files").unwrap();

    if let Some(matches) = matches.subcommand_matches("optimize") {
        optimize::action(matches, files);
    }else if let Some(matches) = matches.subcommand_matches("check") {
        check::action(matches, files);
    }
}

