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

struct Metadata {
    cover_id: String,

    language: String,
    title: String,
    creator: String,
    subjects: Vec<String>,
    date: String,
    description: String,
}

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("optimize")
        .about("optimizes and edits metadata of .epub files")
        .arg(
            Arg::with_name("no-optimize")
                .help("Disables optimization")
                .short("O")
                .long("no-optimize"),
        ).arg(
            Arg::with_name("metadata")
                .help("Edits and rewrites the metadata")
                .short("m")
                .long("metadata"),
        ).arg(
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

pub fn process(path: &str, optimize: bool, metadata: bool, title: Option<&str>) {
    let tmp = unzip(path);
    let basename = Path::new(path).file_stem().unwrap().to_str();
    if metadata {
        mod_metadata(&tmp, basename.unwrap_or("unknown file name"), title);
    }
    if optimize {
        minify(&tmp);
    }
    gen_epub(path, &tmp);
}

fn mod_metadata(tmp: &tempfile::TempDir, default_new_title: &str, title: Option<&str>) {
    println!("Rewriting metadata...");
    let file = File::open(format!(
        "{}/META-INF/container.xml",
        tmp.path().to_str().unwrap()
    )).unwrap();
    let doc = xmltree::Element::parse(file).unwrap();
    let opf = &doc
        .get_child("rootfiles")
        .unwrap()
        .get_child("rootfile")
        .unwrap()
        .attributes["full-path"];

    let mut file = File::open(tmp.path().join(&opf)).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let mut doc =
        xmltree::Element::parse(str::replace(buffer.as_str(), "\u{feff}", "").as_bytes()).unwrap();

    {
        let metadata_ele = doc.get_mut_child("metadata").unwrap();
        let mut metadata = Metadata {
            cover_id: String::new(),

            language: String::new(),
            title: String::new(),
            creator: String::new(),
            subjects: vec![],
            date: String::new(),
            description: String::new(),
        };
        for child in &metadata_ele.children {
            if child.name == "meta" {
                let key = child
                    .attributes
                    .get("name")
                    .unwrap_or(&String::new())
                    .clone();
                let val = child
                    .attributes
                    .get("content")
                    .unwrap_or(&String::new())
                    .clone();
                println!("{}: {}={}", child.name, key, val);

                if key == "cover" {
                    metadata.cover_id = val;
                }
            } else {
                let key = &child.name;
                let val = &child.text.clone().unwrap_or(String::new());
                let val = val.clone();
                println!("{}: {}", key, val);
                match key.as_str() {
                    "language" => metadata.language = val,
                    "title" => metadata.title = val,
                    "creator" => metadata.creator = val,
                    "subject" => metadata.subjects.push(val),
                    "date" => metadata.date = val[0..4].to_owned(),
                    "description" => metadata.description = val,
                    _ => {}
                }
            }
        }

        metadata_ele.children = vec![];
        if metadata.cover_id.len() != 0 {
            let mut ele = xmltree::Element::new("meta");
            ele.attributes = HashMap::with_capacity(2);
            ele.attributes.insert("name".to_owned(), "cover".to_owned());
            ele.attributes
                .insert("content".to_owned(), metadata.cover_id);
            metadata_ele.children.push(ele);
        }

        let prompt = title.is_none();
        let new_title = title.unwrap_or(default_new_title);

        fn modify(dom: &mut xmltree::Element, name: &str, default: String, prompt: bool) {
            let mut input = String::new();
            print!("{} [{}]: ", name, default);
            if prompt {
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_owned();
                if input.len() == 0 {
                    input = default;
                }
            }else{
                input = default;
            }

            let mut ele = xmltree::Element::new(name);
            ele.prefix = Some("dc".to_owned());
            ele.text = Some(input);
            dom.children.push(ele);
        }

        modify(metadata_ele, "language", metadata.language, prompt);
        if prompt {
            modify(metadata_ele, "title", metadata.title, prompt);
        }else{
            modify(metadata_ele, "title", new_title.to_string(), prompt);
        }
        modify(metadata_ele, "creator", metadata.creator, prompt);
        modify(
            metadata_ele,
            "subject",
            if metadata.subjects.len() == 0 {
                String::new()
            } else {
                metadata.subjects[0].clone()
            },
            prompt
        );
        modify(metadata_ele, "date", metadata.date, prompt);
        modify(metadata_ele, "description", metadata.description, prompt);
    }

    let file = File::create(tmp.path().join(&opf)).unwrap();
    doc.write(file).unwrap();
}

fn minify(tmp: &tempfile::TempDir) {
    println!("Minifying files...");
    let mut bytes_saved = 0;
    for entry in WalkDir::new(&tmp) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();

        let ext = path.extension();
        if ext == None {
            continue;
        }
        let ext = ext.unwrap();

        let original_len = entry.metadata().unwrap().len();
        match ext.to_str().unwrap() {
            "opf" | "xml" | "html" | "htm" => {
                Command::new("minify")
                    .arg("--mime=text/xml")
                    .arg(path)
                    .output()
                    .unwrap();
            }
            "css" | "svg" => {
                Command::new("minify")
                    .arg(path)
                    .arg("-o")
                    .arg(path)
                    .output()
                    .unwrap();
            }
            "jpeg" | "jpg" => {
                Command::new("jpegoptim")
                    .arg("-s")
                    .arg(path)
                    .output()
                    .unwrap();
            }
            "png" => {
                Command::new("crunch").arg(path).output().unwrap();
                // FIXME when crunch adds an option to overwrite file
                // https://github.com/chrissimpkins/Crunch/issues/20
                fs::rename(
                    path.parent().unwrap().join(
                        path.file_stem().unwrap().to_str().unwrap().to_owned() + "-crunch.png",
                    ),
                    path,
                ).unwrap();
            }
            _ => {}
        }
        bytes_saved += original_len - entry.metadata().unwrap().len();
        print!("\r{}KiB saved.", bytes_saved / 1024);
        io::stdout().flush().unwrap();
    }
    println!();
}
