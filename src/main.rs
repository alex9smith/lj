#[macro_use]
extern crate clap;

use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::time::SystemTime;
use std::process;
use serde_json::{json, to_value, Value};
use clap::{Arg, App};

fn main() {
    let matches = App::new("lj")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Like `ls`, but outputs JSON!")
        .arg(Arg::with_name("path")
             .help("The file path to list. Defaults to ./")
             .index(1))
        .arg(Arg::with_name("recursive")
            .short("r")
            .long("recursive")
            .help(
                "If set, lj will recursively search directories at the target path for files
                up to a maximum search depth. Set the depth with '-d=', defaults to 5"
            ))
        .arg(Arg::with_name("depth")
            .short("d")
            .takes_value(true)
            .help("The search depth to use when recursively searching folders for files.
                  If set, you must also pass '-r', Defaults to 5."))
        .get_matches();

    // Parse arguments
    let path = matches.value_of("path").unwrap_or("./");
    let recursive = matches.is_present("recursive");
    if !recursive & matches.is_present("depth") {
        eprintln!("Error: You must set '-r' when passing a depth!");
        process::exit(1);
    }
    let depth = value_t!(matches.value_of("depth"), u32).unwrap_or(5);

    let json = dir_to_json(fs::read_dir(path).unwrap(), recursive, depth, 0);
    for entry in json {
        println!("{}", entry)
    }
}

fn dir_to_json(dir: ReadDir, recursive: bool, max_depth: u32, current_depth: u32) -> Vec<Value> {
    let mut json: Vec<Value> = Vec::new();
    for entry in dir {
        let path = entry.unwrap();
        json.push(entry_to_json(path, recursive, max_depth, current_depth))
    }
    json
}

fn entry_to_json(entry: DirEntry, recursive: bool, max_depth: u32, current_depth: u32) -> Value {
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
    let mut result = json!({
        "file_name": file_name,
        "is_dir": metadata.is_dir(),
        "size": metadata.len(),
        "read_only": metadata.permissions().readonly(),
        "last_modified": last_modified,
        "created": created,
    });

    if metadata.is_dir() & (current_depth < max_depth) & recursive {
        //TODO - read the folder properly and pass it in
        let full_path = fs::canonicalize(entry.path()).unwrap();
        result["contents"] = match to_value(dir_to_json(fs::read_dir(full_path).unwrap(), recursive, max_depth, current_depth + 1)) {
            Ok(t) => t,
            Err(_) => Value::Array(Vec::new())
        }
    }

    result
}
