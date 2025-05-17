use crate::dtypes;

use std::collections::{HashMap, VecDeque};

use dtypes::{TaskWrapper, Task, TaskResult};
use crate::dtypes::OutputType;

pub struct Worker {
    tasks: Box<VecDeque<TaskWrapper>>,
    results: HashMap<u32, Box<TaskResult>>,
    queued_tasks: Vec<u32>, 
    curr_id: u32,
    processed: u32
    
}

pub trait ResultRetType: From<i32> + From<u32> + From<String> + Clone + 'static {}

impl Worker {
    pub fn new() -> Self {
        Self {
            tasks: Box::new(VecDeque::new()),
            results: HashMap::new(),
            queued_tasks: Vec::new(),
            curr_id: 0,
            processed: 0
        }
    }

    pub fn add_task<T: Task + 'static>(&mut self, task: T) {
        self.curr_id += 1;
        self.tasks.push_back(
            TaskWrapper::wrap(self.curr_id, Box::new(task))
        );
        self.queued_tasks.push(self.curr_id);
    }

    pub fn run_tasks(&mut self) {
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

    pub fn get_result_wrapper(&self, task_id: u32) -> Option<&TaskResult> {
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
    
    pub fn print_task_result(&self) {
        for x in self.queued_tasks.iter() {
            let task_res = self.get_result_wrapper(*x).unwrap();
            if task_res.ok {
                if task_res.result.is_some() {
                    match task_res.output_type {
                        OutputType::Int => {
                            println!(
                                "task with id={x} produced: {:?}",
                                task_res.get_result::<i32>().unwrap()
                            );
                        }
                        OutputType::UInt => {
                            println!(
                                "task with id={x} produced: {:?}",
                                task_res.get_result::<u32>().unwrap()
                            );
                        }
                        OutputType::StrVal => {
                            println!(
                                "task with id={x} produced: {:?}",
                                task_res.get_result::<String>().unwrap()
                            );
                        }
                        OutputType::VecVal => {
                            println!("not yet supported");
                        }
                        _ => {
                            panic!("this is unexpected!");
                        }
                    }
                } else {
                    println!("task with id={x} passed but does not produce any result!");
                }
            } else {
                println!(
                    "task with id={x} failed! error: {:?}",
                    task_res.error.as_ref().unwrap()
                );
            }
        }
    }
}