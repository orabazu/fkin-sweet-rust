
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

  // getter
  fn full_info(&self) -> String {
    format!("{} {} of House Stark", self.name, self.age.to_string())
  }

  // setter
  fn set_name(&mut self, name: &str) {
    self.name = name.to_string();
  }

  fn to_tuple(self) -> (String, i32){
    (self.name, self.age)
  }
}


  
pub fn run(){
  let mut pers = Person::new("Arya", 17);
  // println!("{:?}", (pers.age, pers.name));
  pers.set_name("Jon");
  println!("{:?}", pers.full_info());
  println!("{:?}", pers.to_tuple());
}