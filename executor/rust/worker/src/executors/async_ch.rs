use crate::dtypes;

use tklog::{error, info};

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use dtypes::{Task, TaskResult, TaskWrapper};

pub struct ListenerWorker {
    sender: Sender<TaskWrapper>,
    worker_handle: Option<thread::JoinHandle<()>>,
    exec_status: Arc<Mutex<HashMap<u32, TaskResult>>>,
    queued_tasks: VecDeque<u32>,
    curr_id: u32,
    processed: Arc<Mutex<u32>>,
}

impl ListenerWorker {
    pub fn new(timeout: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<TaskWrapper>();

        // let results = HashMap::new();
        let queued_tasks = VecDeque::new();
        let curr_id = 0;
        let exec_status = Arc::new(Mutex::new(HashMap::new()));
        let processed = Arc::new(Mutex::new(0));

        let exec_status_cp = Arc::clone(&exec_status);
        let processed_cp = Arc::clone(&processed);

        // Spawn the worker thread
        let worker_handle = thread::spawn(move || {
            loop {
                let result = receiver.recv_timeout(Duration::from_secs(timeout as u64));

                match result {
                    Ok(wrapper) => {
                        let task_result = wrapper.task.run();

                        let mut processed_val = processed_cp.lock().unwrap();
                        *processed_val += 1;

                        let mut status_map = exec_status.lock().unwrap();
                        status_map.insert(wrapper.id, task_result);
                    }
                    Err(_) => {
                        if *processed_cp.lock().unwrap() > 0 {
                            error!("Receiver timed-out!");
                            break;
                        }
                        thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        });

        Self {
            sender,
            worker_handle: Some(worker_handle),
            exec_status: exec_status_cp,
            queued_tasks,
            curr_id,
            processed,
        }
    }

    pub fn listen(&mut self) {
        if let Some(handle) = self.worker_handle.take() {
            handle.join().expect("Something unexpected happened");
        } else {
            println!("Dropping ListenerWorker");
        }
    }

    pub fn add_task<T>(&mut self, task: T)
    where
        T: Task + 'static,
    {
        self.curr_id += 1;
        let wrapper = TaskWrapper {
            id: self.curr_id,
            task: Box::new(task),
        };

        match self.sender.send(wrapper) {
            Ok(()) => {
                self.queued_tasks.push_back(self.curr_id);
            }
            Err(err) => {
                error!("Failed to schedule task: {:?}", err);
            }
        }
    }

    pub fn print_task_result(&self) {
        let status_map = self.exec_status.lock().unwrap();
        for (x, result) in status_map.iter() {
            match result {
                TaskResult::Pass => {
                    info!(format!("PASSED: task with id={} completed!", x));
                }
                TaskResult::Fail(err) => {
                    error!(format!("FAILED: task with id={} failed err: {}", x, err));
                }
            }
        }
    }

    pub fn status(&self) {
        let total = *self.processed.lock().unwrap();
        let failed = self
            .exec_status
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, status)| matches!(status, TaskResult::Fail(_)))
            .count();
        let passed = total - failed as u32;

        info!(format!(
            "Execution status: total={total}, passed={passed}, failed={failed}"
        ));
    }
}

// impl Drop for ListenerWorker {
//     fn drop(&mut self) {
//         if let Some(handle) = self.worker_handle.take() {
//             handle.join().expect("Something unexpected happened");
//         } else {
//             println!("Dropping ListenerWorker");
//         }
//     }
// }
