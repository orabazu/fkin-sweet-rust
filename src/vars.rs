// Vars store primitive data or reference data like js
// Vars are immutable by default
// It is block-scoped like js
pub fn run() {
  let mut name = "Arya";
  let question = "Who are you ?";
  println!("{}", question);
  println!("Girl's name is {}", name);
  name = "No one!";
  println!("{}", question);
  println!("{}", name);

  // Constants
  const ID: i32 = 1;
  println!("{}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Arya", 19);
  println!("{} is {}", my_name, my_age);
}
