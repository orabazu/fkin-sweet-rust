// Array's are fixed length in rust, holds same data type

pub fn run() {
  let numbers: [i8;4] = [1,2,3,4];
  println!("{:?}", numbers);
  println!("Single value of arr: {}", numbers[2]);
  println!("Numbers arr occupies {} bytes", std::mem::size_of_val(&numbers));

  // slice exampl
  let slice: &[i8] = &numbers[1..3];
  println!("Sliced array {:?}", slice);
}
