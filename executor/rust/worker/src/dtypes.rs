pub trait Task: Send + Sync {
    fn run(&self) -> TaskResult;
    fn name(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Pass,
    Fail(String)
}

pub struct TaskWrapper {
    pub id: u32,
    pub task: Box<dyn Task>,
}

impl TaskWrapper {
    pub fn wrap(id: u32, task: Box<dyn Task>) -> Self {
        Self { id, task }
    }
}
