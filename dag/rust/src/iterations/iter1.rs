// Explain it to imaginary duck
use std::fmt::Debug;

/*
Node is like an Abstract base class in Python, only method names are defined. 

Struct that implements this trait must define the functionality. There is no need to specify 
access specifier on methods.
 */
pub trait Node1 {
    /// Implement what should happen when Node is executed
    fn run(&self) -> i32;
}

/*
First define the struct, every struct will have a mandatory field `val` and may have some additional
fields.

Type of `val` depends on struct!
 */
#[derive(Clone)]
pub struct Value {
    pub val: i32,
}


/*
Let's implement Node trait (i.e. define what run and get should do) for Value1.

It's loosely related to implementing abstract methods in Python.
 */
impl Node1 for Value {
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

/// It's nice if we could generate the code for defining what Debug trait should do for Nodes
/// that take multiple values as input.
///
/// # Examples
/// Add(vec!(val1, val2, val3))
macro_rules! impl_debug_for {
    ($struct_name:ident) => {
        impl<'a> std::fmt::Debug for $struct_name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut _debug_str = String::from(stringify!($struct_name));
                _debug_str.push_str("(");

                for node in &self.val {
                    _debug_str.push_str(&format!("{:?}, ", node.run()));
                }

                _debug_str = _debug_str.strip_suffix(", ").unwrap().to_string();
                _debug_str.push_str(")");

                write!(f, "{}", _debug_str)
            }
        }
    };
}


/*
Q. Why Vec needs <Box>?

1. Node is a trait which needs to be implemented by some struct.
2. I am specifying vector accepts any struct that implements Node trait (as type here).
3. But, Vector need to know how much memory to allocate on stack, and thus need to know size of Node at compile time.
4. And, we don't know the actual size at compile time, so we `Box` it i.e. save it on Heap memory.
5. `dyn` keyword specifies that dynamic dispatch. As Node is type and different struct can implement it. Hence `dyn`
   word will determine what to do at runtime.

Q. Why do we need lifetime specifier here?

<TODO> Add here
*/
pub struct Add<'a> {
    /*
    dyn (dynamic dispatch) keyword say that Vector contains any object that implements Node1 can be
    added to this vector and its will be checked dynamically. 
     */
    pub val: Vec<Box<&'a dyn Node1>>,
}

impl<'a> Node1 for Add<'a> {
    fn run(&self) -> i32 {
        self.val.iter().map(|node| node.run()).sum()
    }
}

impl_debug_for!(Add);


pub struct Subtract<'a> {
    pub val: Vec<Box<&'a dyn Node1>>,
}

impl<'a> Node1 for Subtract<'a> {
    fn run(&self) -> i32 {
        let mut _result = self.val.first().unwrap().run();

        for node in self.val.iter().skip(1) {
            _result -= node.run();
        }
        _result
    }
}

impl_debug_for!(Subtract);


pub struct Multiply<'a> {
    pub val: Vec<Box<&'a dyn Node1>>,
}

impl<'a> Node1 for Multiply<'a> {
    fn run(&self) -> i32 {
        self.val.iter().map(|node| node.run()).product()
    }
}

impl_debug_for!(Multiply);


pub struct Power<'a> {
    pub val: Box<&'a dyn Node1>,
    pub power: u32,
}

impl<'a> Debug for Power<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Power(val={}, power={})", self.val.run(), self.power)
    }
}

impl<'a> Node1 for Power<'a> {
    fn run(&self) -> i32 {
        let val = self.val.run();
        val.pow(self.power)
    }
}


pub struct Square<'a> {
    pub val: Box<&'a dyn Node1>,
}

impl<'a> Node1 for Square<'a> {
    fn run(&self) -> i32 {
        self.val.run().pow(2)
    }
}

pub struct Cube<'a> {
    pub val: Box<&'a dyn Node1>,
}


impl<'a> Node1 for Cube<'a> {
    fn run(&self) -> i32 {
        self.val.run().pow(3)
    }
}

//
// pub struct DAG<'a> {
//     pub final_node: Box<&'a dyn Node>
// }
//
// impl<'a> DAG<'a> {
//     pub fn run(&self) -> i32 {
//         self.final_node.run()
//     }
// }