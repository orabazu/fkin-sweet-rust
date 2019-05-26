
struct Person{
  name: String,
  age: i32,
}

impl Person {
  fn new(name: &str, age: i32) ->Person {
    Person{
      name: name.to_string(),
      age:age
    }
  }

  fn full_info(&self) -> String {
    format!("{} {} of House Stark", self.name, self.age.to_string())
  }
}


  
pub fn run(){
  let pers = Person::new("Arya", 17);
  // println!("{:?}", (pers.age, pers.name));
  println!("{:?}", pers.full_info());
}