use clap::Parser;
use serde_json::{to_string_pretty, Value};
use std::collections::HashMap;
use std::path::PathBuf;

#[allow(dead_code, unused_imports)]
#[derive(Parser, Debug)]
#[command(name = "rwjson")]
#[command(version = "1.0")]
#[command(about="rwjson", long_about = None)]
struct Cli {
    #[arg(short, long, help = "read a given json file")]
    read: Option<PathBuf>,

    #[arg(short, long, help = "write / update json file")]
    write: Option<PathBuf>,

    #[arg(short, long, help = "optional for reading, mandatory ")]
    key: Option<String>,
}

fn parse_args() -> Cli {
    let args = Cli::parse();

    let read_file = args.read.as_deref();
    let write_file = args.write.as_deref();
    let key = args.key.as_deref();

    if read_file.is_none() && write_file.is_none() {
        println!("Either --read/-r or --write/-w must be provided!");
        std::process::exit(1);
    }

    if !write_file.is_none() && key.is_none() {
        println!("For --write/-w, --key must be provided!");
        std::process::exit(1);
    }

    if read_file.is_some() && !read_file.unwrap().is_file() {
        println!("File to read '{:?}' is not a file! ", read_file.unwrap());
        std::process::exit(1);
    }

    args
}

fn read(path: PathBuf, key: Option<&str>) {
    let data = std::fs::read_to_string(path).unwrap();
    let data_json = serde_json::from_str::<Value>(&data).unwrap();

    if let Some(key) = key {
        let missing_key = Value::String('-'.to_string());
        let key_val = data_json.get(key).unwrap_or(&missing_key);

        if key_val == &missing_key {
            println!("key: '{}' not found in JSON!", key);
        } else {
            println!("{}: {}", key, to_string_pretty(&key_val).unwrap());
        }
        return;
    }

    println!("{}", to_string_pretty(&data_json).unwrap());
}

fn write(path: PathBuf, keys: HashMap<&str, &str>) {
    let mut key_vals = HashMap::new();

    if path.is_file() {
        let content = std::fs::read_to_string(&path).unwrap();
        key_vals = serde_json::from_str::<HashMap<String, Value>>(&content).unwrap();
    }

    for (key, val) in keys {
        key_vals.insert(key.to_owned(), Value::String(val.to_owned()));
    }

    _ = std::fs::write(path, to_string_pretty(&key_vals).unwrap());
}

fn main() {
    let args = parse_args();

    if args.read.is_some() {
        read(args.read.unwrap(), args.key.as_deref())
    } else if args.write.is_some() {
        let mut keys = HashMap::new();

        for val in args.key.as_deref().clone().unwrap().split(',') {
            if let Some((k, v)) = val.split_once(":") {
                keys.insert(k.trim(), v.trim());
            }
        }

        write(args.write.unwrap(), keys);
    }
}
