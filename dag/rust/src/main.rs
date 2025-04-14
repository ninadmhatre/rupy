mod iterations;

use crate::iterations::iter2::Node2;
use iterations::iter1::Node1;
use iterations::{iter1 as i1, iter2 as i2};
/*
I am trying to solve a simple equation `(a - b)^3 = a^3 - 3a^2b + 3ab^2 - b^3.
*/
fn solution_iter1() {
    let a = i1::Value { val: 10 };
    let b = i1::Value { val: 5 };

    let a_cube = i1::Cube { val: Box::new(&a) };
    let b_cube = i1::Cube { val: Box::new(&b) };

    let a_square = i1::Square { val: Box::new(&a) };
    let b_square = i1::Square { val: Box::new(&b) };

    let three_a_sqr_b = i1::Multiply { val: vec![Box::new(&i1::Value { val: 3 }), Box::new(&a_square), Box::new(&b)] };
    let three_a_b_sqr = i1::Multiply { val: vec![Box::new(&i1::Value { val: 3 }), Box::new(&a), Box::new(&b_square)] };

    let first_part = i1::Subtract { val: vec![Box::new(&a_cube), Box::new(&three_a_sqr_b)] };
    let second_part = i1::Subtract { val: vec![Box::new(&three_a_b_sqr), Box::new(&b_cube)] };
    let final_val = i1::Add { val: vec![Box::new(&first_part), Box::new(&second_part)] };

    println!("(a+b)^3 where a={:?} and b={:?} equals = {}", &a, &b, final_val.run());
}

/*
iter1 is working but it can be improved!. This Rust code is mimicing Python style.

Let's try the second approach which deals with operator overloading, and we can restrict the passing
arbitrary number of arguments to Aggregate nodes (Add, Sub etc)
 */

fn solution_iter2() {
    let a = i2::Value { val: 10 };
    let b = i2::Value { val: 5 };

    let a_cube = i2::Power { val: Box::new(&a), power: 3 };
    let b_cube = i2::Power { val: Box::new(&b), power: 3 };

    let a_square = i2::Power { val: Box::new(&a), power: 2 };
    let b_square = i2::Power { val: Box::new(&b), power: 2 };

    let three_a_sqr_b = &i2::Value { val: 3 } * &a_square * &b;
    let three_a_b_sqr = &i2::Value { val: 3 } * &b_square * &a;

    let final_val = (&a_cube - &three_a_sqr_b) + (&three_a_b_sqr - &b_cube);

    println!("(a+b)^3 where a={:?} and b={:?} equals = {}", &a, &b, final_val.run());
}


fn main() {
    solution_iter1();
    solution_iter2();
}
