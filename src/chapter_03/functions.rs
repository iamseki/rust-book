// functions must have their types declaredddddd 111!!!!!
pub fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("****** FUNCTIONS ******");
  println!("The measurement is: {value}{unit_label}");
}

pub fn statements_and_expressions() {
  println!("****** FUNCTIONS - STATEMENTS n EXPRESSIONS ******");
  // this can't work cause (let y = 6) doesn't return a value
  // let x = (let y = 6);
  let x = 10;
  println!("(let x = 10) -- This is a statement, we simply declare x = 10. x => {x}");

  let y = {
    let x = 7;
    x * 2
  };

  // with semicolons this will turn into a statement and will not return any value!!!
  println!("(let y = {{
    let x = 7;
    7 * 2
  }}) -- This is an expression, note the last line do not include ending semicolons. y => {y}");
}

pub fn return_17() -> i32 {
  17
}

pub fn plus_one(x: i32) -> i32 {
  x + 1
}