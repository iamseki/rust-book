#[path = "./chapter_03/types_variables.rs"]
mod types_variables;

#[path = "./chapter_03/variables_mutability.rs"]
mod variables_mutability;

use variables_mutability::{*};

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
    types_variables:: tuples();
    types_variables::arrays();
    println!("****** Studing rust 1!!! ******");
}
