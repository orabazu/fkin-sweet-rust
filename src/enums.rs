enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_insect(m: Movement){
  // match is similar to switch
  match m {
    Movement::Up => println!("Moving to up"),
    Movement::Down => println!("Moving to down"),
    Movement::Left => println!("Moving to left"),
    Movement::Right => println!("Moving to right")
  }
}

pub fn run(){
  move_insect(Movement::Up);
  move_insect(Movement::Left);
  move_insect(Movement::Left);
  move_insect(Movement::Down);
}