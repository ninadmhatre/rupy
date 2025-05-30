use crate::dtypes;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use tklog::{info, warn};

use crate::dtypes::TaskResult;
use dtypes::Task;

pub struct PrintRows {
    pub rows: u8,
    pub msg: String,
}

impl Task for PrintRows {
    fn run(&self) -> TaskResult {
        for _ in 0..self.rows {
            info!(format!("{}: **** {} ****", self.name(), self.msg));
        }

        TaskResult::Pass
    }

    fn name(&self) -> String {
        "PrintRows".to_string()
    }
}

pub struct AddNums {
    pub a: i32,
    pub b: i32,
}

impl Task for AddNums {
    fn run(&self) -> TaskResult {
        info!(format!(
            "AddTask: {} + {} = {}",
            self.a,
            self.b,
            self.a + self.b
        ));
        TaskResult::Pass
    }

    fn name(&self) -> String {
        "AddNums".to_string()
    }
}

pub struct ReadFile<'a> {
    pub path: &'a Path,
}

impl<'a> ReadFile<'a> {
    pub fn from_string(path_str: &'a str) -> Self {
        let path = Path::new(path_str);

        if !path.is_file() {
            warn!(format!("File {:?} does not exist!", &path));
        }

        Self { path }
    }
}

impl<'a> Task for ReadFile<'a> {
    fn run(&self) -> TaskResult {
        if !self.path.is_file() {
            return TaskResult::Fail(format!("{:?} file does not exist", &self.path).to_string());
        }

        let mut contents = String::new();
        let file_handler = OpenOptions::new().read(true).open(self.path);

        if file_handler.is_err() {
            return TaskResult::Fail(format!(
                "failed to read file :{:?} with err: {:?}",
                &self.path,
                file_handler.err()
            ));
        }

        _ = file_handler.unwrap().read_to_string(&mut contents);

        info!(format!(
            "{}: {:?} has {} lines",
            self.name(),
            self.path,
            self.path.metadata().unwrap().len()
        ));

        TaskResult::Pass
    }

    fn name(&self) -> String {
        "ReadFile".to_string()
    }
}
