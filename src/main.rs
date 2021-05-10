use clap::{Arg, App};
use std::fs;
use std::fs::DirEntry;
use serde_json::json;

fn main() {
    let matches = App::new("lj")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Like `ls`, but outputs JSON!")
        .arg(Arg::with_name("path")
             .help("The file path to list. Defaults to ./")
             .index(1))
        .get_matches();


    let path = matches.value_of("path").unwrap_or("./");
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let path = entry.unwrap();
        println!("{}", Ok(path.file_name().into_string()));
        println!("{}", entry_to_json(path));
    }
}

fn entry_to_json(entry: DirEntry) -> String {
    let metadata = entry.metadata().unwrap();
    let result = json!({
        "file_name": entry.file_name(),
        "is_dir": metadata.is_dir()
    });

    result.to_string()
}
