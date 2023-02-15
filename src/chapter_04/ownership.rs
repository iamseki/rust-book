pub fn stack_and_heap() {
  println!("****** OWNERSHIP - stack n heap ******");
  println!("
Both, stack and the heap are parts of memory available to your code to use at runtime.
--- The stack, stores values in order and every value has a fixed known size and in a LIFO way!!!
--- The heap, stores values in less organized way, and the size is unkown so the allocator must find a space in memmory tha fit its value!!
");
println!("They have different purposes to your code !!!!");
println!("*** RULES => Each value in Rust has an owner.***");
println!("*** RULES => There can only be one owner at a time.***");
println!("*** RULES => When the owner goes out of scope, the value will be dropped.***");
}

pub fn variable_scope(){
  println!("****** OWNERSHIP - Variable Scope ******");
  let s = "hello";
  // println!("s2 => {s2}") is not valid here!!!
  println!("s => {s} is valid in this scope");
  {
    let s2 = "another hello";
    println!("s2 => {s2} declared in the innermost scope and will be valid only inside this scope");
  }
}

pub fn string_type_example() {
  println!("****** OWNERSHIP - Memmory Allocation ******");
  {
  // we will initiate a String type to use the heap instead of the stack
  // this type can be used in situations that we don't know exactly the size of input, like user input
  let mut s = String::from("This is a String");
  // s must be mutable to change it like this:
  s.push_str(", push string");

  println!("this is a string with append \", push string\" at the end => {s}");
  println!("the memory to store this variable must be allocated on the heap at runtime!!");
  }
  // Rust calls drop automatically at the closing curly bracket.
  println!("rust will free the memory of \"s\" when scope is ended and call de drop() method for complex structures!!!"); 
  // both values will be pushed onto the stack, since we know its size!!! its a fixed i32
  // due to the implementation o Copy trait.
  let x = 5;
  let y = x;
  println!("variables with known sizes will be pushed onto the stack: x => {x}, y => {y}");

  let s1 = String::from("test");
  let s2 = s1;
  println!("s2 = s1 will not duplicated the data on the heap, both s1 and s2 will be a pointer to the same heap location");
  // this will not work beacause rust will invalidated s1 due to memory management
  // avoid double free error,
  // println("s1 => {s1}");

  println!("s1 were moved to s2 => {s2}");

  {
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("but we can clone the heap explicity => s3 {s3} => s4 {s4}");
  }
}

fn takes_ownership(some_string: String){
  println!("takes ownership of some_string: {some_string}");
}

fn takes_copy(some_number: i32) {
  println!("takes copy of some_number: {some_number}");
}

pub fn ownership_and_functions() {
  println!("****** OWNERSHIP - Functions copied or moved ******");
  let some_string = String::from("string to take");

  takes_ownership(some_string);
  // some_string is invalid here cause were moved to function argument!!;
  //println!("trying to use some_string => {some_string}");
  println!("some_string is invalid here cause were moved to function argument!! which is nice");

  let some_number = 17;
  takes_copy(some_number);

  println!("some_number still can be used because it was copied !!! nice =: {some_number}");
}

pub fn return_values_and_scope() {
  println!("****** OWNERSHIP - return values scopes ******");
  let s1 = gives_ownership(); // gives ownership

  let s2 = String::from("hello");

  println!("s1 is valid => {s1}");
  let s3 = takes_and_gives_back(s2);
  // s2 is not valid anymore, were moved to s3
  // println!("s2 is invalid => {s2}");
  println!("s3 => {s3}, now at the end of the scope memory for all variables will be freable to allocator");
}

fn gives_ownership() -> String {
  let some_string = String::from("yours");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}