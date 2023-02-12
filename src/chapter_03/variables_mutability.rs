use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

pub fn mutability() {
  println!("****** MUTABILITY ******");
  // this wont work due to the variable is immutable by default
  // let x = 5;
  // we have to explicity declare a mutable variable like this
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");
}

pub fn constants() {
  println!("****** CONSTANTS ******");
  // const can be only immutable and the type must be annotated this wont work:
  // const three_hours_in_second = 60 * 60 * 3;
  // also is a convention user UPPERCASE_WORDS separated by underscore:
  // the compiler also is capable to evalute the result of this expression at compilation time
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}

pub fn shadowing() {
  println!("****** SHADOWING ******");

  // we can shadow a variable by using the same variable's name and repeating the use of the let keyword as follows:
  let x = 5;

  let x = x + 1;

  // in this inner scope the value of 6 of x is used
  {
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}");
      // now x is 12 but in the end of this inner scope the x becomes 6 again...
  }

  println!("The value of x is: {x}");

  // this wont work due to: mismatched types
  // let mut spaces = "   ";
  // spaces = spaces.len();
  // with shadowing we effectively creates a new variable, so we can do this:
  let spaces = "   ";
  println!("Tha value of var spaces: \"{spaces}\", the type: {}", type_of(spaces));
  let spaces = spaces.len();
  println!("Tha value of var spaces: {spaces}, the type: {}", type_of(spaces));
}

