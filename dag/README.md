
# dag

----

### Description

Create a lazy DAG (Directed Acyclic Graph) lib, to solve simple mathematical equation involving add, subtract, multiply operations.

Few constrains:
1. All nodes in DAG will produce a wrapper over result in order to make it lazy. Please see example for clarity.
2. Final value will be i32.
3. Nodes with signature to build,
   4. Value(a: i32) -> i32
   5. Sum(a: Value, b: Value) -> Value(i32)
   6. Minus(a: Value, b: Value) -> Value(i32)
   7. Multiply(a: Value, b: Value) -> Value(i32)
   8. Power(a: Value, power: u32) -> Value(i32)
4. DAG is lazy, to get the final result, call `.run()` to get the result.

### Example

----

```python
from nodes import Value, Sum, Minus, Multiply, Power

# Solve (a + b)^2 = a^2 + 2ab + b^2

a = Value(10)
b = Value(5)

a_square = Power(a, power=2)
b_square = Power(b, power=2)
two_ab = a * b * Value(2)

result = a_square + two_ab + b_square

print(f"({a} + {b})^2 = {result.run()})   # 225 
```

In the code, I am trying to solve "(a - b)^3 = a^3 - 3a^2b + 3ab^2 - b^3" in the rust and python example.

Note: Python example is not yet (06-Mar-25) added. I will soon add it for reference.