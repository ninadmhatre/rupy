#[allow(unused)]
mod dtypes;
mod tasks;
mod executor;

use executor::Executor;
use tasks::{PrintRows, AddNums};

fn main() {
    let mut exec = Executor::new();
    
    let t1 = PrintRows { rows: 1, msg: "Hello".to_string()};
    let t2 = AddNums { a: 100, b: 999};
    
    let t1_id = exec.add(t1);
    let t2_id = exec.add(t2);
    
    println!("Pending tasks: {}", exec.pending());
    
    exec.run();
    
    println!("Pending tasks now, {}", exec.pending());
    
    let t1_res = exec.get_result(t1_id).unwrap();
    println!("t1_result is : {:?}", t1_res);

    let t2_res = exec.get_result(t2_id).unwrap();
    println!("Was t2 successful? : {}", t2_res.ok);
    println!("t2_Result is: {:?}", t2_res.get_result::<u32>().unwrap());
    
}
