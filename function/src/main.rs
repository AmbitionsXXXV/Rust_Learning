// #![allow(clippy::approx_constant)]
// #[warn(clippy::no_effect)]

use std::f64::consts;

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
  f(value)
}

fn square(value: i32) -> i32 {
  value * value
}

fn cube(value: i32) -> i32 {
  value * value * value
}

fn pi() -> f64 {
  // 3.1415926
  consts::PI
}

fn not_pi() {
  3.1415926
}

fn main() {
  let is_pi = pi();
  let is_unit1 = not_pi();
  let is_unit2 = {
    pi();
  };

  println!(
    "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
    is_pi,
    is_unit1,
    is_unit2,
  );
  println!("apply square: {}", apply(2, square));
  println!("apply cube: {}", apply(2, cube));
}
