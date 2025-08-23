use regex_style::{vim_regex, Converter, CONVERTER};
use clap::{Arg,App};

fn main() {
    let matches = App::new("regex-style")
        .about("regular express style converter")
        .version("0.1.0")
        .arg(Arg::with_name("output-style")
            .short("o")
            .long("output-style")
            .help("outpu style")
            .possible_values(&["emca","perl"])
            .takes_value(true)
            .default_value("emca")
            .required(true))
        .arg(Arg::with_name("expression")
            .help("regular expression with style of vimscript")
            .index(1)
            .required(true))
        .get_matches();

    if matches.is_present("expression"){
        let exp = matches.value_of("expression").unwrap();
        println!("exp:{}", exp);

        let ostyle = matches.value_of("output-style").unwrap();
        unsafe {
            CONVERTER = match ostyle {
                "emca" => Converter::EmcaConverter,
                "perl" => Converter::PerlConverter,
                _ => Converter::EmcaConverter,
            }
        }
        vim_regex::pattern(exp).unwrap();
    }
}
