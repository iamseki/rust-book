pub fn simple_nested_control_flow() {
    println!("****** CONTROL FLOW ******");
    let number = 3;

    // if number {} do not work !!! must be a boolean type
    if number != 0 {
        println!("number {number} is not equal to zero");
    } else if number == 4 {
        println!("can be nested as well");
    } else {
        println!("and this is the last statement!");
    }
}

pub fn ternary_if_like(flag: bool) -> i32 {
    // if and else expression must return the same type !!!!!
    if flag {
        2
    } else {
        3
    }
}

pub fn loop_control() {
    println!("****** CONTROL FLOW - loops ******");
    let mut counter = 0;

    loop {
        if counter == 2 {
            break;
        }
        counter += 1;
        println!("counter value => {counter}");
    }

    // it can returns an statement on break that will be assigned to "result" variable 
    let result = loop {
      counter += 1;
      if counter == 10 {
        break counter * 2;
      }
    };

    println!("result of loop => {result}");
}


pub fn labeled_loop() {
  println!("****** CONTROL FLOW - labeled loop ******");
  // we can use the break and continue keywords in specific loops
  // instead of innermost loop

  let mut count = 0;
  'counting_up: loop {
    println!("count {count}");
    let mut remaining = 10;

    loop {
      println!("remaining {remaining}");
      // we first break this innermost loop
      // note that also remaining will becomes "10" again when this loop ends
      if remaining == 9 {
        break;
      }
      // and when count == 2 we break the counting_up labeled loop and ends this function, awesome!!!
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");
}

pub fn while_loop() {
  println!("****** CONTROL FLOW - while condition ******");

  let mut number = 3;

  while number != 0 {
    println!("{number}!");
    number -= 1;
  }
}

pub fn for_in_loop() {
  println!("****** CONTROL FLOW - for in ******");

  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {element}");
  }

  // we can also iterates in a range like this (for number in (1..4).rev());
  for number in (1..4).rev() {
    println!("{number} !!");
  }
}
