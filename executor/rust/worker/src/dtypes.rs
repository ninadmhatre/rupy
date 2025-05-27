use std::any::Any;

// #[derive(Debug, Clone)]
// pub enum OutputType {
//     Null,
//     Int,
//     UInt,
//     StrVal,
//     VecVal,
// }

pub trait Task: Send + Sync {
    fn run(&self) -> TaskResult;
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Pass,
    Fail(String)
}

// impl TaskResult {
//     pub fn pass_flag(output_type: OutputType) -> Self {
//         Self {
//             ok: true,
//             result: None,
//             error: None,
//             output_type,
//         }
//     }
// 
//     pub fn pass(result: Box<dyn Any + Send + Sync>, output_type: OutputType) -> Self {
//         Self {
//             ok: true,
//             result: Some(result),
//             error: None,
//             output_type,
//         }
//     }
// 
//     pub fn fail(error: String) -> Self {
//         Self {
//             ok: false,
//             result: None,
//             error: Some(error),
//             output_type: OutputType::Null,
//         }
//     }
// 
//     pub fn get_result<R: 'static + Clone>(&self) -> Result<R, String> {
//         match &self.result {
//             Some(result) => match result.downcast_ref::<R>() {
//                 Some(value) => Ok(value.clone()),
//                 None => Err("Could not downcast to the requested type".to_string()),
//             },
//             None => Err("No result available".to_string()),
//         }
//     }
// 
//     pub fn get_error(&self) -> Option<String> {
//         match &self.error {
//             Some(msg) => Some(msg.clone()),
//             None => None,
//         }
//     }
// }

pub struct TaskWrapper {
    pub id: u32,
    pub task: Box<dyn Task>,
}

impl TaskWrapper {
    pub fn wrap(id: u32, task: Box<dyn Task>) -> Self {
        Self { id, task }
    }
}
