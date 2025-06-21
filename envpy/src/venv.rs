// region - import-core
use std::path::PathBuf;
use std::process::Command;

// endregion - import-core

// region - import-self-crate
use crate::parser::{Actions, Cli};
use crate::shell::{BashProfile, ProfileRenamer};
// endregion - import-self-crate

pub struct VEnv {
    pub args: Cli,
}

impl VEnv {
    fn setup(&self, handler: impl ProfileRenamer) -> i32 {
        handler.setup()
    }

    fn create(&self) -> i32 {
        if PathBuf::from(&self.args.dir_name).exists() {
            eprintln!("{} named file/dir already exists!", self.args.dir_name);
            return 1;
        }

        let venv_cmd = Command::new("python3")
            .args(["-m", "venv", self.args.dir_name.as_str()])
            .output();

        if venv_cmd.is_err() {
            eprintln!("failed to create virtual env [python -m venv failed]");
            return 1;
        }

        let unwrapped = venv_cmd.unwrap();
        if !unwrapped.status.success() {
            eprintln!("failed to create virtual env {:?}", &unwrapped.stderr);
            return 1;
        }

        0
    }

    fn rename(&self, handler: impl ProfileRenamer) -> i32 {
        match handler.rename(self.args.backup) {
            Ok(_) => 0,
            Err(msg) => {
                eprintln!("failed to rename virtual env! {:?}", msg);
                1
            }
        }
    }

    fn get_handlers(&self) -> impl ProfileRenamer {
        match BashProfile::new(&self.args.actions, &self.args.dir_name, &self.args.name) {
            Ok(handler) => handler,
            Err(msg) => {
                eprintln!("failed to get handler {msg}");
                std::process::exit(1)
            }
        }
    }
    pub fn run(&self) -> i32 {
        let handler = self.get_handlers();

        match self.args.actions {
            Actions::Setup => self.setup(handler),
            Actions::Create => {
                let create_rc = self.create();
                if create_rc == 0 {
                    self.rename(handler)
                } else {
                    create_rc
                }
            }
            Actions::Rename => self.rename(handler),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_run() {
//         let venv = VEnv {
//             args: Cli {
//                 actions: Actions::Setup,
//                 dir_name: String::from("test"),
//                 name: String::from("test"),
//                 backup: false,
//             },
//         };
//     }
// }
