use std::any::Any;
pub trait Task {
    fn run(&self) -> TaskResult;
}

#[derive(Debug)]
pub struct TaskResult {
    pub ok: bool,
    pub result: Option<Box<dyn Any>>,
    pub error: Option<String>
}

impl TaskResult {
    pub fn no_result() -> Self {
        Self {
            ok: true,
            result: None,
            error: None
        }
    }
    
    pub fn no_error(result: Box<dyn Any>) -> Self {
        Self {
            ok: true,
            result: Some(result),
            error: None
        }
    }
    
    pub fn get_result<R: 'static + Clone>(&self) -> Result<R, String> {
        match &self.result {
            Some(result) => {
                match result.downcast_ref::<R>() {
                    Some(value) => Ok(value.clone()),
                    None => Err("Could not downcast to the requested type".to_string())
                }
            },
            None => Err("No result available".to_string())
        }
    }
    
    pub fn get_error(&self) -> Option<String> {
        match &self.error {
            Some(msg) => Some(msg.clone()),
            None => None
        }
    }
}

pub struct TaskWrapper {
    pub id: u32,
    pub task: Box<dyn Task>
}

impl TaskWrapper {
    pub fn wrap(id: u32, task: Box<dyn Task>) -> Self {
        Self { id, task }
    }
}