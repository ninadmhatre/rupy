use worker::dtypes::*;

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
        TaskResult::pass(Box::new(self.num.ilog(self.base)), OutputType::UInt)
    }
}