use crate::dtypes;

use std::collections::{HashMap, VecDeque};

use dtypes::{TaskWrapper, Task, TaskResult};

pub struct Executor {
    tasks: Box<VecDeque<TaskWrapper>>,
    results: HashMap<u32, Box<TaskResult>>,
    curr_id: u32,
    processed: u32
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: Box::new(VecDeque::new()),
            results: HashMap::new(),
            curr_id: 0,
            processed: 0
        }
    }
    
    pub fn add<T: Task + 'static>(&mut self, task: T) -> u32 {
        self.curr_id += 1;
        self.tasks.push_back(
            TaskWrapper::wrap(self.curr_id, Box::new(task))
        );
        self.curr_id
    }
    
    pub fn run(&mut self) {
        while self.tasks.len() > 0 {
            match self.tasks.pop_front() {
                Some(wrapper) => {
                    let id = wrapper.id;
                    let task_result = wrapper.task.run();
                    
                    self.processed += 1;
                    self.results.insert(id, Box::new(task_result));
                },
                None => println!("No more tasks to execute!")
            }
        }
    }
    
    pub fn get_result(&self, task_id: u32) -> Option<&TaskResult> {
        match self.results.get(&task_id) {
            Some(val) => {
                Some(val.as_ref())
            },
            None => None
        }
    }
    
    pub fn pending(&self) -> usize {
        self.tasks.len()
    }
}