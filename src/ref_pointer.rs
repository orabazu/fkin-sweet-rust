
pub fn run(){
  // For non primitive vars use & to borrow it
  // &str read only borrow, ownership is not changed
  // &mut str mutable borrow, ownership is not changed
  let my_vec = vec![1,2,3];
  let my_vec2 = &my_vec;
  println!("{:?}",&my_vec );
}