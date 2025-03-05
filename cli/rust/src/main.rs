use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use shellexpand::tilde;

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

fn get_platform_defaults(platform: &str) -> Value {
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
fn profile_dir(platform: &str) -> PathBuf {
    let prof_dir = get_platform_defaults(platform)["profile_dir"]
        .as_str()
        .unwrap()
        .to_string();

    PathBuf::from(tilde(&prof_dir).to_string())
}

fn profile_file(platform: &str) -> PathBuf {
    PathBuf::from(profile_dir(platform)).join("env.json")
}

fn read_profile_file(platform: &str) -> Value {
    let profile_file = profile_file(platform);

    if !profile_file.exists() {
        println!("Error: Profile file {:?} does not exist", profile_file);
        exit(1);
    }

    let file_content = fs::read_to_string(profile_file).unwrap();
    let parsed: Value = serde_json::from_str(&file_content).unwrap();
    json!(parsed)
}

fn write_profile_file(platform: &str, data: Value) {
    let profile_file = profile_file(platform);
    fs::write(profile_file, data.to_string()).unwrap();
}

// Commands
fn init_cmd(platform: &str, recreate: bool, def_profile: &str) {
    let template = json!({
        "profiles": {
            "common": {"A": 1, "B": true, "C": "rw"},
            "dev": {"HOST": "a", "NAME": "DB"},
            "dev-lite": {"LOCAL_DB_DIR": "/tmp/rupy", "LDB_VAR1": "r", "LDB_VAR2": "w"},
            "uat": {"LOCAL_DB_DIR": null, "HOST": null, "NAME": null},
            "default": def_profile,
        }
    });

    let profile_file = profile_file(platform);
    if profile_file.exists() && !recreate {
        println!("Error: {:?} already exist!", profile_file);
        return;
    }

    fs::create_dir_all(profile_dir(platform).as_path()).unwrap();
    write_profile_file(platform, template);
}

fn load_cmd(platform: &str, profile: &str) {
    let env_data = read_profile_file(platform);

    let profile_name = if profile == "" {
        env_data["profiles"]["default"].to_string()
    } else {
        profile.to_string()
    };

    if !env_data["profiles"]
        .as_object()
        .unwrap()
        .contains_key(&profile_name)
    {
        println!("Error: Profile '{:?}' does not exist!", profile_name);
        return;
    }

    let common_vars = env_data["profiles"]["common"].as_object().unwrap();
    let selected_vars = env_data["profiles"][profile_name].as_object().unwrap();

    let mut merged = HashMap::<String, Value>::new();
    merged.extend(common_vars.clone());
    merged.extend(selected_vars.clone());

    merged.into_iter().for_each(|(k, v)| {
        if v.is_null() {
            println!("unset {k}");
        } else {
            println!("export {k}={}", v.to_string());
        }
    })
}

fn debug_cmd(platform: &str, profile: &str) {
    let env_data = read_profile_file(platform);
    let profile_name = if profile == "" {
        env_data["profiles"]["default"].to_string()
    } else {
        profile.to_string()
    };

    println!(
        "Available Profile [{:?}]",
        env_data["profiles"].as_object().unwrap()
    );
    println!("Loading values for [{:?}]", profile_name);
    println!("-------");

    load_cmd(platform, &profile_name);
}

// Other
fn get_platform() -> String {
    if env::var("ON_GCP").unwrap_or("".to_string()) == "1" {
        return "gcp".to_string();
    }
    "linux".to_string()
}

fn main() {
    let _cli = Cli::parse();

    let platform = get_platform();
    
    match _cli.command {
        Commands::Load { profile } => {
            load_cmd(&platform, &profile);
        },
        Commands::Init { recreate, default_profile } => {
            println!("init with recreate={}, default_profile={}", recreate, default_profile);
            init_cmd(&platform, recreate, &default_profile);
        },
        Commands::Debug { profile } => {
            println!("debug called with {}", profile);
            debug_cmd(&platform, &profile);
        }
    }
}
