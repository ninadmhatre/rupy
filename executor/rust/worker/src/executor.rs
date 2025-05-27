use std::cell::RefCell;
use crate::dtypes;

use tklog::{info, warn, error};

use std::collections::{HashMap, VecDeque};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread;
use std::time::Duration;
use dtypes::{TaskWrapper, Task, TaskResult};
// use crate::dtypes::OutputType;

pub struct Worker {
    tasks: VecDeque<TaskWrapper>,
    exec_status: HashMap<u32, TaskResult>,
    queued_tasks: Vec<u32>,
    curr_id: u32,
    processed: u32
}

/*
1. Add worker::with_channels(ideal_timeout: 100sec) to return sender
2. New tasks can be scheduled by sending it worker
3. Tasks will be executed once worker.run is called
4. Print result will be used to print the result
 */



impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            exec_status: HashMap::new(),
            queued_tasks: Vec::new(),
            curr_id: 0,
            processed: 0
        }
    }
    
    pub fn add_task<T>(&mut self, task: T)
        where T: Task + 'static
    {
        self.curr_id += 1;
        self.tasks.push_back(
            TaskWrapper::wrap(self.curr_id, Box::new(task))
        );
        self.queued_tasks.push(self.curr_id);
    }

    fn run_task(&mut self, wrapper: TaskWrapper) {
        let id = wrapper.id;
        let task_result = wrapper.task.run();

        self.processed += 1;
        self.exec_status.insert(id, task_result);
    }

    pub fn run_tasks(&mut self) {
        while !self.tasks.is_empty() {
            match self.tasks.pop_front() {
                Some(wrapper) => {
                    self.run_task(wrapper);
                },
                None => println!("No more tasks to execute!")
            }
        }
    }

    pub fn get_result_wrapper(&self, task_id: u32) -> Option<&TaskResult> {
        match self.exec_status.get(&task_id) {
            Some(val) => {
                Some(val)
            },
            None => None
        }
    }

    pub fn pending(&self) -> usize {
        self.tasks.len()
    }
    
    pub fn print_task_result(&self) {
        for x in self.queued_tasks.iter() {
            match self.get_result_wrapper(*x).unwrap() {
                TaskResult::Pass => {
                    info!(format!("PASSED: task with id={} completed!", x));
                },
                TaskResult::Fail(err) => {
                    error!(format!("FAILED: task with id={} failed err: {}", x, err));
                }
            }
        }
    }
}


// pub struct ListenerWorker {
//     sender: SyncSender<TaskWrapper>,
//     worker_handle: Option<thread::JoinHandle<()>>,
//     exec_status: Mutex<HashMap<u32, TaskResult>>,
//     queued_tasks: Mutex<Vec<u32>>,
//     curr_id: Mutex<u32>,
//     timeout: u32,
//     buffer: u32,
// }
// 
// impl ListenerWorker {
//     pub fn new(timeout: u32, buffer: u32) -> Self {
//         // Create a channel with the specified buffer size
//         let (sender, receiver) = mpsc::sync_channel(buffer as usize);
// 
//         // Create shared state
//         let results = Mutex::new(HashMap::new());
//         let queued_tasks = Mutex::new(Vec::new());
//         let curr_id = Mutex::new(0);
// 
//         // Clone the sender for the worker thread
//         let worker_sender: SyncSender<TaskWrapper> = sender.clone();
// 
//         // Spawn the worker thread
//         let worker_handle = thread::spawn(move || {
//             for wrapper in receiver {
//                 // Process the task
//                 let task_result = wrapper.task.run();
// 
//                 // Store the result
//                 let mut results = results.lock().unwrap();
//                 results.insert(wrapper.id, Box::new(task_result));
// 
//                 // Remove from queued tasks
//                 let mut queued = queued_tasks.lock().unwrap();
//                 if let Some(pos) = queued.iter().position(|&id| id == wrapper.id) {
//                     queued.remove(pos);
//                 }
//             }
//         });
// 
//         Self {
//             sender,
//             worker_handle: Some(worker_handle),
//             exec_status: ,
//             queued_tasks,
//             curr_id,
//             timeout,
//             buffer,
//         }
//     }
// 
//     pub fn add_task<T: Task + 'static>(&self, task: T) -> u32 {
//         // Generate new task ID
//         let mut id_guard = self.curr_id.lock().unwrap();
//         *id_guard += 1;
//         let task_id = *id_guard;
//         drop(id_guard);  // Release the lock as soon as possible
// 
//         // Create the wrapper
//         let wrapper = TaskWrapper {
//             id: task_id,
//             task: Box::new(task),
//         };
// 
//         // Add to queued tasks
//         self.queued_tasks.lock().unwrap().push(task_id);
// 
//         // Send the task to the worker thread
//         if let Err(e) = self.sender.send(wrapper) {
//             eprintln!("Failed to send task to worker: {}", e);
//             // Remove from queued tasks if sending failed
//             self.queued_tasks.lock().unwrap().retain(|&id| id != task_id);
//             return 0;  // Or handle error appropriately
//         }
// 
//         task_id
//     }
// 
//     pub fn get_result_mut(&self, task_id: u32) -> Option<&Box<TaskResult>> {
//         let result  = self.results.lock().unwrap();
//         result.get(&task_id)
//     }
// }
// 
// impl Drop for ListenerWorker {
//     fn drop(&mut self) {
//         // Dropping the sender will cause the receiver's iterator to end
//         // and the worker thread will exit its loop
//         if let Some(handle) = self.worker_handle.take() {
//             if let Err(e) = handle.join() {
//                 eprintln!("Error joining worker thread: {:?}", e);
//             }
//         }
//     }
// }
// pub struct ListenerWorker {
//     sender: Sender<TaskWrapper>,
//     receiver: Arc<Mutex<Receiver<TaskWrapper>>>, // Arc<Mutex<Receiver<TaskWrapper>,
//     worker_handle: Option<thread::JoinHandle<()>>,
//     timeout: u32,
//     tasks: VecDeque<TaskWrapper>,
//     results: HashMap<u32, Box<TaskResult>>,
//     queued_tasks: Vec<u32>,
//     curr_id: u32,
//     processed: u32,
//     buffer: u32,
// }
//
// impl ListenerWorker {
//     pub fn new(timeout: u32, buffer: u32) -> Self {
//         let (sender, receiver) = mpsc::channel();
//
//         Self {
//             sender,
//             receiver: Arc::new(Mutex::new(receiver)),
//             worker_handle: None,
//             timeout,
//             tasks: VecDeque::new(),
//             results: HashMap::new(),
//             queued_tasks: Vec::new(),
//             curr_id: 0,
//             processed: 0,
//             buffer
//         }
//     }
//
//     pub fn listen(&mut self) {
//         let receiver = self.receiver.clone();
//         // let mut result = self.results;
//
//         let handle = thread::spawn(move || {
//             loop {
//                 let task = {
//                     let rx = receiver.lock().unwrap();
//                     rx.try_recv()
//                 };
//
//                 match task {
//                     Ok(wrapper) => {
//                         self.run_task(wrapper);
//                     },
//                     Err(mpsc::TryRecvError::Empty) => {
//                         thread::sleep(Duration::from_secs(1))
//                     },
//                     Err(mpsc::TryRecvError::Disconnected) => {
//                         eprintln!("Sender disconnected, shutting down!!");
//                         break;
//                     }
//                 }
//             }
//         });
//
//         // self.worker_handle = Some(handle);
//     }
//
//     pub fn add_task<T>(&mut self, task: T)
//     where
//         T: Task + 'static
//     {
//
//         self.curr_id += 1;
//         let wrapped = TaskWrapper::wrap(self.curr_id, Box::new(task));
//         if let Err(e) = self.sender.send(wrapped) {
//             eprintln!("Failed to send task to worker {}", e);
//         } else {
//             self.queued_tasks.push(self.curr_id);
//         }
//     }
//
//     fn run_task(&self, wrapper: TaskWrapper) {
//         let id = wrapper.id;
//         let task_result = wrapper.task.run();
//
//         self.processed += 1;
//         self.results.insert(id, Box::new(task_result));
//     }
//
//     pub fn run_tasks(&mut self) {
//         while !self.tasks.is_empty() {
//             match self.tasks.pop_front() {
//                 Some(wrapper) => {
//                     self.run_task(wrapper);
//                 },
//                 None => println!("No more tasks to execute!")
//             }
//         }
//     }
// }