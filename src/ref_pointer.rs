
pub fn run(){
  // For non primitive vars use & to point it.
  let my_vec = vec![1,2,3];
  let my_vec2 = &my_vec;
  println!("{:?}",&my_vec );
}