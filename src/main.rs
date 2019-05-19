mod printer;
mod vars;

// This is the main function
fn main() {
  let name = String::from("Arya");
  printer::cannonical_print(name);
  printer::pretty_print();
  printer::print_complex_number();
  vars::run();
}

