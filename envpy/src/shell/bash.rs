// region -- import-core
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::{fs, io};
// endregion -- import-core

// region -- import-extern
use regex::Regex;
// endregion -- import-extern

// region -- import-crate
use crate::parser::Actions;
use crate::shell::ProfileRenamer;
// endregion -- import-crate

pub struct BashProfile<'a> {
    pub dir: &'a String,
    pub name: &'a String,
    pub venv_file_name: String,
    pub prof_file_name: String,
}

impl<'a> BashProfile<'a> {
    pub fn new(action: &Actions, dir: &'a String, name: &'a String) -> Result<Self, String> {
        let obj = Self {
            dir,
            name,
            venv_file_name: String::from("activate"),
            prof_file_name: String::from(".bashrc"),
        };

        if !obj.get_profile_path().is_file() && action == &Actions::Rename {
            return Err(format!(
                "non existing path '{:?}' specified!",
                obj.get_profile_path()
            ));
        }

        Ok(obj)
    }

    // TODO: not happy with this
    // pub(crate) fn for_test(
    //     dir: &'a String,
    //     name: &'a String,
    //     prof_file_name: &'a String,
    //     venv_file_name: &'a String,
    // ) -> Self {
    //     Self {
    //         dir,
    //         name,
    //         venv_file_name: String::from(venv_file_name),
    //         prof_file_name: String::from(prof_file_name),
    //     }
    // }

    fn get_profile_path(&self) -> PathBuf {
        let home = std::env::home_dir();

        if home.is_none() {
            eprintln!("home dir is not setup for user, setup failed!");
            std::process::exit(1)
        }

        home.unwrap().join(&self.prof_file_name)
    }
}

impl<'a> ProfileRenamer for BashProfile<'a> {
    fn get_path(&self) -> PathBuf {
        let venv_activate_file = format!("{}/bin/{}", self.dir, self.venv_file_name);
        PathBuf::from(venv_activate_file)
    }

    fn rename(&self, backup: bool) -> io::Result<()> {
        if self.name.is_empty() {
            eprintln!(
                "please provide a new name with '-n | --name' for the virtual env, nothing provided!"
            );
            return Ok(());
        }

        let path = self.get_path();

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("venv activate file: '{}' does not exist", path.display()),
            ));
        }

        let new_path = path.with_extension(".new");

        let src_file = File::open(&path).expect("failed to read file");
        let src_reader = BufReader::new(src_file);

        let mut dst_file = File::create(&new_path)?;
        let regex = Regex::new(r#"^\s*PS1="\(.*\) \$\{PS1"#).unwrap();

        for line in src_reader.lines() {
            let l = line?;

            if regex.is_match(&l) || l.contains("VIRTUAL_ENV_PROMPT") {
                let new_line = l.replace(self.dir, self.name);
                writeln!(dst_file, "{}", new_line)?;
            } else {
                writeln!(dst_file, "{}", l)?;
            }
        }

        if backup {
            fs::rename(&path, path.with_extension(".bak"))?;
        }

        fs::rename(&new_path, path)?;

        Ok(())
    }

    fn already_setup(&self) -> u8 {
        let mut rc = 0;

        match File::open(self.get_profile_path()) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    let l = line.unwrap();

                    if l.trim_start().starts_with("function cd()") {
                        rc = 1;
                        break;
                    }
                }
            }
            Err(msg) => {
                eprintln!("failed to open profile! err: {:?}", msg);
                rc = 9;
            }
        }

        rc
    }

    fn setup(&self) -> i32 {
        let cd_function = r#"
function cd() {
    command cd $@ || return
    
    if [[ -d .venv ]] || [[ -e .venv ]]; then
        if declare -F deactivate > /dev/null; then
           deactivate
        fi
        
        source .venv/bin/activate
        echo "** activated current venv **" 
    fi
    
    if [[ -f .cdenv ]]; then
        echo "** loading .cdenv **"
        source ./.cdenv
    fi
"#;
        let mut rc = 1;

        match self.already_setup() {
            0 => {
                let mut profile_file = OpenOptions::new()
                    .append(true)
                    .open(self.get_profile_path())
                    .unwrap();

                match writeln!(profile_file, "{}", cd_function) {
                    Ok(_) => {
                        println!("updated profile file: {:?}", self.get_profile_path());
                        rc = 0
                    }
                    Err(msg) => {
                        eprintln!(
                            "failed to write setup function in profile file! err: {:?}",
                            msg
                        );
                    }
                }
            }
            1 => {
                println!("already setup!");
                rc = 0
            }
            _ => eprintln!("profile missing or unexpected error!"),
        }

        rc
    }
}
