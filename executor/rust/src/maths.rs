use worker::dtypes::*;
use tklog::{info};

pub struct Log {
    pub num: u32,
    pub base: u32,
}

impl Log {
    pub fn new(num: u32) -> Self {
        Self {
            num,
            base: 2
        }
    }
    
    pub fn with_base(num: u32, base: u32) -> Self {
        Self {
            num , base
        }
    }
}


impl Task for Log {
    fn run(&self) -> TaskResult {
        let result = self.num.ilog(self.base);
        info!(format!("Log {} of {} = {}", self.base, self.num, result));
        
        TaskResult::Pass
    }

    fn name(&self) -> String {
        "Log".to_string()
    }
}