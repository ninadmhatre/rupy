use std::process;

use clap::Parser;

use envpy::{Cli, VEnv};

fn main() {
    let args = Cli::parse();

    let venv = VEnv { args };
    process::exit(venv.run());
}
