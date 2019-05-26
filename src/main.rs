mod printer;
mod vars;
mod types;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod ref_pointer;
mod structs;

// This is the main function
fn main() {
  let name = String::from("Arya");
  printer::cannonical_print(name);
  printer::pretty_print();
  printer::print_complex_number();
  vars::run();
  types::run();
  tuples::run();
  arrays::run();
  vectors::run();
  // conditionals::run();
  // loops::run();
  functions::run();
  ref_pointer::run();
  structs::run();

}

