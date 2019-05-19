use std::fmt; // Import `fmt`
pub fn cannonical_print(first_name: String) {
  println!("My name is {0}, {1} {0}", "Stark", first_name);
  let pi = 3.141592;
  let formatted_number = format!("{:.*}", 3, pi);
  println!("Pi is roughly {}", formatted_number);
  println!("Question mark format for trait {:?}", (true, 25, "Sandor"));
}

pub fn pretty_print() {
  #[derive(Debug)]
  struct Person<'a> {
    name: &'a str,
    age: i32,
  }
  let name = "Arya";
  let age = 19;
  let peter = Person { name, age };

  // Pretty print
  println!("{:#?}", peter);
}



// A custom implementation of print for Struct
pub fn print_complex_number() {

  #[derive(Debug)]
  pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
  }

  // Similarly, implement for Point2D
  impl fmt::Display for Complex<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Customize so only `x` and `y` are denoted.
      write!(f, "real: {}, imag: +{}i", self.re, self.im)
    }
  }

  let complex_number = Complex { re: 3.3, im: 7.2 };
  println!("Compare points:");
  println!("Display: {}", complex_number);
  println!("Debug: {:?}", complex_number);
}
