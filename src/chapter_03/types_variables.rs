use std::mem;

pub fn parse() {
    println!("****** PARSE ******");
    // types, rust must know every variable type at compile time!
    let guess = "42".parse::<u32>().expect("Not a number1!");
    println!("parse is great! u32 parse::<u32> => {guess}");

    let guess: f64 = "64".parse().expect("not a number !!!");
    println!("parse is great again! parse explicity with anotation parse() => {guess}");
}

pub fn integers_scalar_types() {
    println!("****** INTEGER TYPES ******");
    let signed_int128: i128 = 10;
    let u_int128: u128 = 29;
    println!("signed_int128: {signed_int128}, u_int128: {u_int128}");
    println!(
        "size of signed_int128 is {} bytes",
        mem::size_of_val(&signed_int128)
    );
    println!("size of u_int128 is {} bytes", mem::size_of_val(&u_int128));

    // the default is signed int 32
    let number = 27;
    println!(
        "size of the default int32 type number is {} bytes",
        mem::size_of_val(&number)
    );

    // we can also infere the type like these
    let number = 789u32;
    println!(
        "size of number of explicity u32 number is {} bytes",
        mem::size_of_val(&number)
    );

    // omg, we can use _ as a visual separator!
    let number = 1_997;
    println!(
        "number with separator is {number} and size {} bytes",
        mem::size_of_val(&number)
    );

    // we can also write integer literals !!!
    let number = 0xff;
    println!(
        "number in hexadecimal is {number} and the size is {} bytes",
        mem::size_of_val(&number)
    );
    let number = 0o77;
    println!(
        "number in octal is {number} and the size is {} bytes",
        mem::size_of_val(&number)
    );
    // and mix with the visual separator!!!!!
    let number = 0b1111_1111;
    println!(
        "number in binary is {number} and the size is {} bytes",
        mem::size_of_val(&number)
    );
    // this is a BYTE literal omg!
    let number = b'A';
    println!(
        "number in Byte is {number} and the size is {} bytes",
        mem::size_of_val(&number)
    );
}

pub fn float_scalar_types() {
    println!("****** FLOAT TYPES ******");
    let x = 2.0;
    println!(
        "this is a float number: {x} which size is {} bytes and default to f64",
        mem::size_of_val(&x)
    );
    let x = 2.0f32;
    println!(
        "this is a float number: {x} which size is {} bytes explicity to 32 bits float number",
        mem::size_of_val(&x)
    );
}

pub fn numeric_operations() {
    println!("****** NUMERIC OPERATIONS ******");
    let sum = 2 + 15;
    println!("this is sum => {sum}");

    let difference = 17.7 - 5.0;
    println!("this is a subtraction => {difference}");

    let product = 2 * 7;
    println!("this is a multiplication => {product}");

    let quotient = 56.7 / 37.2;
    println!("this is a division (56.7/37.2) => {quotient}");

    let truncated = 5 / 3;
    println!("this is a division (5/3) => {truncated}");
    let truncated = -5 / 3;
    println!("this is a division (-5/3) => {truncated}");
    println!("truncated because of the data type!!!! nice");

    let remainder = 43 % 5;
    println!("this is the remainder of (43 % 5) => {remainder}");
}

pub fn boolean_types() {
    println!("****** BOOLEAN TYPE ******");
    let t = true; // has 1 byte;
                  // ------> Because the CPU can't address anything smaller than a byte.
    println!(
        "this is a bool: {t} which have in size {} byte(s)",
        mem::size_of_val(&t)
    );
}

pub fn character_types() {
    println!("****** CHAR TYPE ******");
    // note the SINGLE QUOTES !!
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // always have four bytes, which means it can represent a lot more than just ASCII !!!!!!
    println!(
        "this is a char {c} which have in size {} bytes",
        mem::size_of_val(&c)
    );
    println!(
        "this is a cat(char) {heart_eyed_cat} which have in size {} bytes",
        mem::size_of_val(&heart_eyed_cat)
    );
    println!("and this is a stranger char {z}...")
}

pub fn tuples() {
    println!("****** COMPOUND TYPES - TUPLES ******");
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size
    // if not explicity the types the defaults will be used foreach type of value....
    let tup = (500, 6.4, 1);

    println!(
        "The size of the tuple is {} bytes",
        mem::size_of_val(&tup)
    );

    // we can also using pattern matching to destructure!!! awesome
    let (x, y, z) = tup;
    println!("This is the x: {x}, the y: {y} and the z: {z}");

    // also we be acessed as position of tuple, such as: tup.0, tup.1 and so on
    println!(
        "Another way to access: tup.0 = {}, tup.1 = {}",
        tup.0, tup.1
    );
}

pub fn arrays() {
    println!("****** COMPOUND TYPES - ARRAYS ******");
    // also have a fixed length like tuples but every element of an array must have the same type!!!!
    // the data will be allocated on the STACK rather than HEAP :thinking:
    let a = [1, 2, 3, 4, 5];

    println!("Array of integers first position: {}", a[0]);

    // will have [1, 1, 1, 1, 1];
    let a = [1; 5];
    println!("Can be declared as [1; 5] which will have five times 1 elements... length => {}", a.len());

    println!("Size in bytes => {}", mem::size_of_val(&a));
    // accessing an index that doesn't exist at runtime execution of course will crash the program!!
}