mod maths;
use std::thread;
use std::time::Duration;

use worker::executors::{async_ch, sync};
use worker::tasks::{AddNums, PrintRows, ReadFile};

use maths::Log;

#[allow(unused)]
fn sync_worker() {
    let mut worker = sync::Worker::new();

    worker.add_task(AddNums { a: 100, b: 999 });
    worker.add_task(PrintRows {
        rows: 1,
        msg: "Hello".to_string(),
    });
    worker.add_task(ReadFile::from_string(
        "/home/ninad/Documents/rupy/executor/rust/Cargo.toml",
    ));
    worker.add_task(Log::with_base(100, 10));
    worker.add_task(Log::new(16));

    println!("Pending tasks: {}", worker.pending());

    worker.run_tasks();

    println!("Pending tasks now, {}", worker.pending());

    worker.print_task_result();
}

fn async_worker() {
    let mut worker = async_ch::ListenerWorker::new(2);

    thread::sleep(Duration::from_secs(3));

    worker.add_task(AddNums { a: 100, b: 999 });
    worker.add_task(PrintRows {
        rows: 1,
        msg: "Hello".to_string(),
    });
    worker.add_task(ReadFile::from_string(
        "/home/ninad/Documents/rupy/executor/rust/Cargo.toml",
    ));
    worker.add_task(Log::with_base(100, 10));
    worker.add_task(Log::new(16));

    for i in 1..10 {
        worker.add_task(AddNums { a: 99, b: i });
    }

    worker.listen();
    worker.status();
}

fn main() {
    // sync_worker();
    async_worker();
}
