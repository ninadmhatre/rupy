use crate::dtypes;

use tklog::{error, info};

use std::collections::{HashMap, VecDeque};

use dtypes::{Task, TaskResult, TaskWrapper};

pub struct Worker {
    tasks: VecDeque<TaskWrapper>,
    exec_status: HashMap<u32, TaskResult>,
    queued_tasks: Vec<u32>,
    curr_id: u32,
    processed: u32,
}

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
            processed: 0,
        }
    }

    pub fn add_task<T>(&mut self, task: T)
    where
        T: Task + 'static,
    {
        self.curr_id += 1;
        self.tasks
            .push_back(TaskWrapper::wrap(self.curr_id, Box::new(task)));
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
                }
                None => println!("No more tasks to execute!"),
            }
        }
    }

    pub fn get_result_wrapper(&self, task_id: u32) -> Option<&TaskResult> {
        match self.exec_status.get(&task_id) {
            Some(val) => Some(val),
            None => None,
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
                }
                TaskResult::Fail(err) => {
                    error!(format!("FAILED: task with id={} failed err: {}", x, err));
                }
            }
        }
    }
}
