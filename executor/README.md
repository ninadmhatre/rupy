## executor

---

Create an executor that,

1. Accept tasks
2. Execute those tasks
3. Return output of tasks
4. <Maybe> Support adding tasks on the fly
5. <Maybe> Cancelling task


For this use case, i am directly going to implement only Rust.

### Example

```rust
let mut executor = Executor::new();

let task1 = PrintRows { row_count: 10, msg: "hello" };
let tast2 = AddNumbers { a: 5, b: 10 };
let task3 = GetUrl { url: "https://some-server/v1/returns/json" };

let t1_i = executor.add_task(task1);  // Trait Task
let t2_i = executor.add_task(task2);
let t3_i = executor.add_task(task3);

println!("Pending tasks: {}", executor.pending);

executor.run();

let task3_op = exec.get_result(t3_i);  // TaskResult

```
