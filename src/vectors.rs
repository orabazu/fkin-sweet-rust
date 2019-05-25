// Vectors are resizable arrs

pub fn run(){
  let mut numbers_vector : Vec<i32> = vec![1,2,3,4];

  // print part is pretty much same with arrays
  println!("{:?}", numbers_vector);
  println!("Single value of vec: {}", numbers_vector[2]);
  println!("Numbers vec occupies {} bytes", std::mem::size_of_val(&numbers_vector));

  // Adding and popping last elem
  numbers_vector.push(2);
  numbers_vector.pop();
  println!("Vector is same after push and pop{:?}", numbers_vector);

   // Looping through wout idx
  for elem in numbers_vector.iter(){
    println!("{}", elem);
  }

  // Looping through
  for (elem,i) in numbers_vector.iter().enumerate(){
    println!("{}, {}", elem,i);
  }

   // Looping through and map
  for elem in numbers_vector.iter_mut(){
    *elem += 2;
  }
    println!("After mutation {:?}", numbers_vector);
}