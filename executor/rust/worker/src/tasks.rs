use crate::dtypes;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use crate::dtypes::TaskResult;
use dtypes::{OutputType, Task};

pub struct PrintRows {
    pub rows: u8,
    pub msg: String,
}

impl Task for PrintRows {
    fn run(&self) -> TaskResult {
        for _ in 0..self.rows {
            println!("**** {} ****", self.msg);
        }

        TaskResult::pass_flag(OutputType::Null)
    }
}

pub struct AddNums {
    pub a: i32,
    pub b: i32,
}

impl Task for AddNums {
    fn run(&self) -> TaskResult {
        TaskResult::pass(Box::new(self.a + self.b), OutputType::Int)
    }
}

pub struct ReadFile<'a> {
    pub path: &'a Path,
}

impl<'a> ReadFile<'a> {
    pub fn from_string(path_str: &'a str) -> Self {
        let path = Path::new(path_str);

        if !path.is_file() {
            println!("Warning: File {:?} does not exist!", &path)
        }

        Self { path }
    }
}

impl<'a> Task for ReadFile<'a> {
    fn run(&self) -> TaskResult {
        if !self.path.is_file() {
            return TaskResult::fail(format!("{:?} file does not exist", &self.path).to_string());
        }

        let mut contents = String::new();
        let file_handler = OpenOptions::new().read(true).open(&self.path);

        if file_handler.is_err() {
            return TaskResult::fail(format!(
                "failed to read file :{:?} with err: {:?}",
                &self.path,
                file_handler.err()
            ));
        }

        _ = file_handler.unwrap().read_to_string(&mut contents);

        TaskResult::pass(Box::new(contents), OutputType::StrVal)
    }
}
