extern crate serde;
extern crate toml;

extern crate clap;
use clap::{Arg,App};
extern crate num_format;
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;

mod files;
use files::get_files;
mod counting;
use counting::{Stats, Counter, get_counters, get_stats};


fn show_stats(stats: &HashMap<String, Stats>, _num: usize) {
    for (ext, stat) in stats.iter(){
        // println!("-----");
        // println!("File Ext: {}", ext);
        // println!("Total files: {}", stat.files_count.to_formatted_string(&Locale::en));
        // println!("Total loc: {}", stat.total_loc.to_formatted_string(&Locale::en));
        // println!("Empty loc: {}", stat.empty_loc.to_formatted_string(&Locale::en));

        // eprintln!("-----{:-^10}-----", ext);
        // print!("{}", toml::to_string(&stat).unwrap());
    }
    let mut vstats: Vec<&Stats> = stats.values().collect();
    vstats.sort_by(|a,b| b.total_loc.partial_cmp(&a.total_loc).unwrap());
    for st in vstats {
        println!("===== File type: {} =====", st.ext);
        println!("Total files: {}", st.files_count.to_formatted_string(&Locale::en));
        println!("Total loc: {}", st.total_loc.to_formatted_string(&Locale::en));
        println!("Empty loc: {}", st.empty_loc.to_formatted_string(&Locale::en));
    }

    // println!("{}", toml::to_string(&stats).unwrap());
}

fn show_counters(counters: &Vec<Counter>, num: usize) {
    //const SHOW_COUNT: usize = num;
    let len = counters.len();
    let max = if len < num {
        len
    }
    else{
        num
    };
    if max > 0 {
        eprintln!("{} biggest files:", max);
    }
    let mut i = 0;
    while i < max { 
        println!("{position}. {total_loc} loc in {file_name}",
                 position = (i + 1),
                 total_loc = counters[i].total_loc,
                 file_name = counters[i].file);
        i += 1;
    } 
}

fn main() {
    eprintln!("Source lines of code program...");
    let matches = App::new("Source lines of code")
        .version("1.0")
        .author("hejack0207 <hejack0207@sina.com>")
        .about("Source lines of codes program")
        .arg(Arg::with_name("summary")
             .short("s")
             .long("summary")
             .help("Display only summary one line")
             .takes_value(false))
        .arg(Arg::with_name("num")
             .short("n")
             .long("number")
             .help("Display only top number")
             .takes_value(true)
             .default_value("10"))
        .arg(Arg::with_name("directory")
             .help("directory to stat")
             .index(1))
        .get_matches();

    let onlysummary = matches.is_present("summary"); //matches.value_of("summary").unwrap_or(false);
    eprintln!("summary flag:{}",onlysummary);
    //let mut directory : &str = ".";
    let mut directory = ".";
    if let Some(ref dir) = matches.value_of("directory") {
        directory = dir;
        eprintln!("directory:{}",dir);
    }

    let mut files: Vec<(String,String)> = Vec::new();
    get_files(directory, &mut files);
    let counters = get_counters(files);
    let stats = get_stats(&counters);
    
    if ! onlysummary {
        if let Some(num_str) = matches.value_of("num"){
            let num = num_str.parse::<usize>();
            if ! num.is_err() {
                if let Ok(num) = num {
                    show_counters(&counters, num);
                }
            }else{
                eprintln!("invalid option value --num:{}",num_str);
            }
        }else{
            eprintln!("invalid option value --num:{:?}",matches.value_of("num"));
        }
    }
    show_stats(&stats, 3);
}

#[test]
fn parse_opt() {
    let m = App::new("myprog")
    .arg(Arg::with_name("summary")
        .short("s"))
    .get_matches_from(vec![
        "myprog2", "-s"
    ]);

    assert!(m.is_present("summary"));
}

#[test]
fn get_files_test() {
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files);
    assert_eq!(2, files.len());
}

#[test]
fn get_counters_test() {
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files);
    let counters = get_counters(files);
    assert_eq!(2, counters.len());
}

#[test]
fn get_stats_test(){
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files); 
    let counters = get_counters(files);
    let stats = get_stats(&counters);

    assert_eq!(2, stats.files_count);
    assert_eq!("10", stats.total_loc);
    assert_eq!(12, stats.empty_loc);
}
