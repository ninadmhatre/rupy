mod maths;

#[allow(unused)]
use worker;
use worker::executor::Worker;
use worker::tasks::{AddNums, PrintRows, ReadFile};

use maths::Log;

// Changes
// Worker: 
// 1. takes tasks
// 2. run them
// 3. Task gives only status (Pass, Failed)

fn main() {
    let mut worker = Worker::new();
    
    worker.add_task(AddNums { a: 100, b: 999 });
    worker.add_task(PrintRows { rows: 1, msg: "Hello".to_string() });
    worker.add_task(ReadFile::from_string("/home/ninad/Documents/rupy/executor/rust/Cargo.toml"));
    worker.add_task(Log::with_base(100, 10));
    worker.add_task(Log::new(16));
    
    println!("Pending tasks: {}", worker.pending());

    worker.run_tasks();
    
    println!("Pending tasks now, {}", worker.pending());
    
    worker.print_task_result();
}
