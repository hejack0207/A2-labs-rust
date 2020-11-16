use clap::{App, Arg, SubCommand, ArgMatches, Values};
// use epub::doc::EpubDoc;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("check")
        .about("check epub info")
        .arg(
            Arg::with_name("metadata")
                .help("show epub metadata info")
                .short("m")
                .long("metadata"),
        )
        .arg(
            Arg::with_name("toc")
                .help("show epub toc")
                .long("toc"),
        )
}

pub fn action(matches: &ArgMatches, files: Values){
    eprintln!("running check subcommand!");
    let metadata = matches.is_present("metadata");
    let toc = matches.is_present("toc");

    use epub::doc::EpubDoc;

    for path in files {
        eprintln!("filepath: {}", path);
        let doc = EpubDoc::new(path);
        assert!(doc.is_ok());
        let doc = doc.unwrap();

        if metadata {
            for (name, value) in &doc.metadata {
                println!("{}: {:?}", name, value);
            }
        }
        if toc {
            for np in &doc.toc {
                println!("{}. {} {:?}", np.play_order, np.label, np.content);
            }
        }
    }
}
