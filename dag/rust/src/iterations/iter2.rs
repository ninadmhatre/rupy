// Explain it to imaginary duck
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

pub trait Node2 {
    fn run(&self) -> i32;
}

/*
First define the struct, every struct will have a mandatory field `val` and may have some additional
fields.

Type of `val` depends on struct!
 */
#[derive(Clone, PartialEq, Copy)]
pub struct Value {
    pub val: i32,
}

/*
Operator overloading Rust way!

Read more: https://doc.rust-lang.org/core/ops/
*/

/*
I do not like what I have done below, I think this is not the Rust way to do this. 

I am basically implementing, 
1. Value + Value
2. Value + &Value
3. &Value + &Value
4. &Value + Value
5. &Value + &Power
6. &Power + &Value

I can implement macro to do this, but I am sure this is not the best way to do this.
 */
impl Add for &Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: self.run() + rhs.run()
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: self.run() + rhs.run()
        }
    }
}


impl Add<&Power<'_>> for &Value {
    type Output = Value;

    fn add(self, rhs: &Power<'_>) -> Self::Output {
        Self::Output { val: self.run() + rhs.run() }
    }
}

impl Add<&Value> for &Power<'_> {
    type Output = Value;
    fn add(self, rhs: &Value) -> Self::Output {
        Self::Output { val: self.run() + rhs.run() }
    }
}

impl Sub for &Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: self.run() - rhs.run()
        }
    }
}

impl Sub<&Power<'_>> for &Value {
    type Output = Value;
    fn sub(self, rhs: &Power<'_>) -> Self::Output {
        Self::Output { val: self.run() - rhs.run() }
    }
}

impl Sub<&Value> for &Power<'_> {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Self::Output {
        Self::Output {
            val: self.run() - rhs.run()
        }
    }
}

impl Mul for &Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: self.run() * rhs.run()
        }
    }
}

impl Mul<&Power<'_>> for &Value {
    type Output = Value;

    fn mul(self, rhs: &Power<'_>) -> Self::Output {
        Self::Output { val: self.run() * rhs.run() }
    }
}

impl Mul<&Value> for &Power<'_> {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Self::Output {
        Self::Output { val: self.run() * rhs.run() }
    }
}

impl Mul<&Value> for Value {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Self::Output {
        Self::Output { val: self.val * rhs.val }
    }
}

/*
Let's implement Node trait (i.e. define what run and get should do) for Value1.

It's loosely related to implementing abstract methods in Python.
 */
impl Node2 for Value {
    /// Returns the value as-is!
    fn run(&self) -> i32 {
        self.val
    }
}

/*
Why implement debug when you can simply `#[derive(Debug)]`?
- To control what you print when you print the type!

It's like __repr__ equivalent in Python.
 */
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // You only define what to return, below line will return the val as-is!
        write!(f, "{}", self.val)
    }
}

pub struct Power<'a> {
    pub val: Box<&'a dyn Node2>,
    pub power: u32,
}

impl<'a> Debug for Power<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Power(val={}, power={})", self.val.run(), self.power)
    }
}

impl<'a> Node2 for Power<'a> {
    fn run(&self) -> i32 {
        let val = self.val.run();
        val.pow(self.power)
    }
}
