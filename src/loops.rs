
pub fn run(){
  let mut count = 0;

  // Endless loop
  loop{
    count += 1;
    println!("{}", count);

    if count == 20 {
      break;
    }
  }

  
  while count <= 40 {
    if count % 3 == 0  && count % 5 == 0{
      println!("fizzbuzz")
    }
    else if count % 3 == 0{
      println!("fizz")
    }
    else if count % 5 == 0{
      println!("buzz")
    }

    count += 1;
  }

  // for range
  for count in 40..60{
     if count % 3 == 0  && count % 5 == 0{
      println!("fizzybuzzy")
    }
    else if count % 3 == 0{
      println!("fizzy")
    }
    else if count % 5 == 0{
      println!("buzzy")
    }
  }
}