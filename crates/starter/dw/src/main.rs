use clap::{arg, Command};
use log::{info, error};
use env_logger;

fn main() {
    env_logger::init();
    let app = Command::new("dw")
        .about("dwarf utilities")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("l")
                .about("list")
                .arg(arg!(<file>"file to process"))
                .arg(arg!(-t --tag "show DW_TAG info").action(clap::ArgAction::SetTrue))
                .arg_required_else_help(true)
        );

    let matches = app.get_matches();

    match matches.subcommand(){
        Some(("l", sub_matches)) => {
            let show_tags = sub_matches.get_flag("tag");
            if show_tags {
                self::show_tags();
            }
        }
        _ => unreachable!(),
    }
}

fn show_tags(){

}
