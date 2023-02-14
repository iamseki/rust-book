#[path = "./chapter_03/types_variables.rs"]
mod types_variables;

#[path = "./chapter_03/functions.rs"]
mod functions;

#[path = "./chapter_03/control_flow.rs"]
mod control_flow;

#[path = "./chapter_03/variables_mutability.rs"]
mod variables_mutability;

use variables_mutability::{*};
use functions::{print_labeled_measurement, statements_and_expressions, return_17, plus_one};


fn main() {
    // variables and mutability chapter, immutable is the default!
    mutability();
    constants();
    shadowing();

    // types, rust must know every variable type at compile time!
    // we can references the pub stuff of the mod with MOD_NAME::SOMESTUFF, very nice!
    types_variables::parse();
    types_variables::integers_scalar_types();
    types_variables::float_scalar_types();
    types_variables::numeric_operations();
    types_variables::boolean_types();
    types_variables::character_types();
    types_variables::tuples();
    types_variables::arrays();

    // functions, their types must be declared!
    // Statements and Expressions can affect functions body in a distinguishing way.
    print_labeled_measurement(17, 'h');
    statements_and_expressions();
    println!("return_17() => {}", return_17());
    println!("5 plus 1 => {}", plus_one(5));
    println!("****** Studing rust 1!!! ******");

    // control flowwww if every where!
    control_flow::simple_nested_control_flow();
    println!("returns 2 if true => {}", control_flow::ternary_if_like(true));
    control_flow::loop_control();
    control_flow::labeled_loop();
    control_flow::while_loop();
    control_flow::for_in_loop();
}
