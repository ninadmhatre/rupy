use std::path::PathBuf;
use crate::dtypes;

use dtypes::Task;
use crate::dtypes::TaskResult;

pub struct PrintRows {
    pub rows: u8,
    pub msg: String
}


impl Task for PrintRows {
    fn run(&self) -> TaskResult {
        for _ in 0..self.rows {
            println!("**** {} ****", self.msg);
        }
        
        TaskResult::no_result()
    }
}


pub struct AddNums {
    pub a: u32,
    pub b: u32
}

impl Task for AddNums {
    fn run(&self) -> TaskResult {
        TaskResult::no_error(Box::new(self.a + self.b))
    }
}


// pub struct ReadFile {
//     pub path: PathBuf,
//     pub lines: usize
// }
// 
// impl Task for ReadFile {
//     fn run(&self) -> TaskResult {
//         if !self.path.is_file() {
//             return TaskResult { ok: false, result: None, error: Some(format!("{} does not exist!", &self.path)}
//         }
//         
//         
//     }
// }