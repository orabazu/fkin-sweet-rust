pub fn run() {
  println!("{}", sum(44, 52));

  // closure sum
  let closure_sum = |x: i32, y: i32| x + y;
  println!("Closure sum: {}", closure_sum(44, 52));
}

fn sum(x: i32, y: i32) -> i32 {
  // don not semicolon to return;
  x + y
}
