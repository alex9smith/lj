use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::time::SystemTime;
use serde_json::json;
use clap::{Arg, App};

fn main() {
    let matches = App::new("lj")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Like `ls`, but outputs JSON!")
        .arg(Arg::with_name("path")
             .help("The file path to list. Defaults to ./")
             .index(1))
        .get_matches();


    let path = matches.value_of("path").unwrap_or("./");
    let json = dir_to_json(fs::read_dir(path).unwrap());
    for entry in json {
        println!("{}", entry)
    }
}

fn dir_to_json(dir: ReadDir) -> Vec<String> {
    let mut json: Vec<String> = Vec::new();
    for entry in dir {
        let path = entry.unwrap();
        json.push(entry_to_json(path))
    }
    json
}

fn entry_to_json(entry: DirEntry) -> String {
    let metadata = entry.metadata().unwrap();
     // Handle cases where there might be an error
    let file_name = match entry.file_name().into_string() {
        Ok(f) => f,
        Err(_) => "".to_string()
    };
    let last_modified = match metadata.modified() {
        Ok(t) => t,
        Err(_) => SystemTime::UNIX_EPOCH
    };
    let created = match metadata.created() {
        Ok(t) => t,
        Err(_) => SystemTime::UNIX_EPOCH
    };
    let result = json!({
        "file_name": file_name,
        "is_dir": metadata.is_dir(),
        "size": metadata.len(),
        "read_only": metadata.permissions().readonly(),
        "last_modified": last_modified,
        "created": created,
    });

    result.to_string()
}
