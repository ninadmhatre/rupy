use clap::{Parser, Subcommand};
use serde::__private::de::InPlaceSeed;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(propagate_version = true)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize profile
    Init {
        #[arg(short, long, default_value_t = false, help = "Recreate profile?")]
        recreate: bool,

        #[arg(short, long, default_value = "local", help = "Specify Default Profile")]
        default_profile: String,
    },
    /// Load given profile
    Load {
        #[arg(short, long, required = true, help = "Profile to load?")]
        profile: String,
    },
    /// View given profile details
    Debug {
        #[arg(short, long, required = true, help = "Profile to debug?")]
        profile: String,
    },
}

fn get_platform_defaults(platform: String) -> Value {
    let default = json!({
        "gcp": {
            "profile_dir": "/GCP/.rupy",
            "root_dir": "/GCP/user/rupy",
            "cache_dir": "/GCP/user/rupy/cache",
            "metrics_dir": "/GCP/user/rupy/metrics"
        },
        "linux": {
            "profile_dir": "~/.rupy",
            "root_dir": "~/rupy",
            "cache_dir": "~/rupy/cache",
            "metrics_dir": "~/rupy/metrics"
        }
    });

    default[platform].clone()
}

// Helpers
fn profile_dir(platform: String) -> PathBuf {
    let prof_dir = get_platform_defaults(platform)["profile_dir"]
        .as_str()
        .unwrap()
        .to_string();

    PathBuf::from(prof_dir)
}

fn profile_file(platform: String) -> PathBuf {
    PathBuf::from(profile_dir(platform)).join("env.json")
}

fn read_profile_file(platform: String) -> Value {
    let profile_file = profile_file(platform);

    if !profile_file.exists() {
        panic!("Profile file {:?} does not exist", profile_file);
    }

    let file_content = fs::read_to_string(profile_file).unwrap();
    let parsed: Value = serde_json::from_str(&file_content).unwrap();
    json!(parsed)
}

fn write_profile_file(platform: String, data: Value) {
    let profile_file = profile_file(platform);
    fs::write(profile_file, data.to_string()).unwrap();
}

// Commands
fn init_cmd(platform: String, recreate: bool, def_profile: String) {
    let template = json!({
        "profiles": {
            "common": {"A": 1, "B": true, "C": "rw"},
            "dev": {"HOST": "a", "NAME": "DB"},
            "dev-lite": {"LOCAL_DB_DIR": "/tmp/rupy", "LDB_VAR1": "r", "LDB_VAR2": "w"},
            "uat": {"LOCAL_DB_DIR": null, "HOST": null, "NAME": null},
            "default": def_profile,
        }
    });

    let profile_file = profile_file(platform.clone());
    if profile_file.exists() && !recreate {
        println!("Error: {:?} already exist!", profile_file);
        return;
    }

    fs::create_dir_all(profile_dir(platform.clone()).as_path()).unwrap();
    write_profile_file(platform, template);
}

// fn _get_set_or_default_profile(platform: &String, default: &String) -> String {
//
// }

fn load_cmd(platform: String, profile: String) {
    let env_data = read_profile_file(platform.clone());

    let profile_name = if profile == "" {
        env_data["profiles"]["default"].to_string()
    } else {
        profile
    };

    if !env_data["profiles"]
        .as_object()
        .unwrap()
        .contains_key(&profile_name)
    {
        println!("Error: Profile '{:?}' does not exist!", profile_name);
        return;
    }

    let common_vars = env_data["profile"]["common"].as_object().unwrap();
    let selected_vars = env_data["profiles"][profile_name].as_object().unwrap();

    let mut merged = HashMap::<String, Value>::new();
    merged.extend(common_vars.clone());
    merged.extend(selected_vars.clone());

    merged.into_iter().for_each(|(k, v)| {
        if v.is_null() {
            println!("unset {:?}", k);
        } else {
            println!("export {:?}='{:?}'", k, v);
        }
    })
}

fn debug_cmd(platform: String, profile: String) {
    let env_data = read_profile_file(platform.clone());
    let profile_name = if profile == "" {
        env_data["profiles"]["default"].to_string()
    } else {
        profile
    };

    println!(
        "Available Profile [{:?}]",
        env_data["profiles"].as_object().unwrap()
    );
    println!("Loading values for [{:?}]", profile_name);
    println!("-------");

    load_cmd(platform.clone(), profile_name.clone());
}

// Handlers
// fn gcp_handler(args: Cli) {
//     match args.command {
//         Commands::Init {
//             recreate,
//             default_profile,
//         } => init_cmd,
//         Commands::Load { profile } => {}
//     }
// }

// fn linux_handler(args: Cli) {}

// Other
fn get_platform() -> String {
    if env::var("ON_GCP").unwrap_or("".to_string()) == "1" {
        return "gcp".to_string();
    }
    "linux".to_string()
}

// fn get_command_handler() -> fn(cli: &Cli) -> Result<(), String> {
//     let platform = get_platform();
//
//     match platform.as_str() {
//         "gcp" => gcp_handler,
//         "linux" => linux_handler,
//         _ => panic!("Unknown platform: {}", platform),
//     }
// }

fn main() {
    let _cli = Cli::parse();
    println!("{:?}", _cli.command);
}
