/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
  let i: i32 = 256;
  let truthy: bool = 10 > 5;
  let smiley = '\u{1F600}';

  println!("{:?}", (i, truthy, smiley));

  // Primitive str = Immutable fixed-length string somewhere in memory
  // String = Growable, heap-allocated data structure - Use when you need to modify or own string data
  let mut name = String::from("Jon");
  // Push char
  name.push('S');
  // Push string
  name.push_str("now");
  // Get length
  println!("Length: {}", name.len());
  // Contains
  println!("Contains 'Snow' {}", name.contains("Snow"));

  // Loop through string by whitespace
  for (i,word) in name.chars().enumerate() {
    println!("{:?}", (word,i));
  }
}
