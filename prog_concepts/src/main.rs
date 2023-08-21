use std::io;
use std::num::Wrapping;

fn mutability() {
    println!("======= MUTABILITY =========");
    // let x = 5;
    // Without `mut`, variables may be treated as `const`
    // Therefore, they may not be reassigned without `mut`
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");
}

fn variables() {
    // The `_` is used to avoid the unused variable check.
    const _THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;

    // Constants are like C primitives - valid for entire run of program
    // NOTE: Only in the scope which they are defined (incl global)
}

fn shadowing() {
    println!("======= SHADOWING =========");

    // #1 Same types, re-assignment

    //first instantiation
    let x = 5;

    // Second instantiation OVER the first
    // >> FOR the rest of the program, it will refer to this one.
    let x = x + 1;

    {
        // Third one - only in THIS scope.
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in the inner scope is {x}");

    // #2 Type Switching :)

    // This is ok.
    let _spaces = "    ";
    let _spaces = _spaces.len(); // `free` the earlier reference, and point it to this type.

    // This is not.
    // let mut _spaces = "     ";
    // _spaces = _spaces.len(); // This attempts to store a variable of a different type without
    //     `free`-ing the reference first. (`mut` reference)
}

fn types() {
    println!("=========== TYPES ============");

    // 1. `parse` requires type annotations
    // let _guess = "42".parse().expect("Not a number");
    let _guess: i32 = "42".parse().expect("Not a number");

    // 2. Wrapping arithmetic to prevent overflow errors
    // let _unwrapped = 0u32 - 1u32; // Overflows due to unsigned
    let num = Wrapping(0u32) - Wrapping(1u32);
    println!(
        "Wrapping - Unsigned (0 - 1) op gives max u32: {}",
        num.eq(&Wrapping(u32::MAX))
    );

    // 3. Floating Point / Standard Precision Operations
    let _sum = 5 + 10; //15
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // -1

    // 4. Booleans (1 Bit) and Chars (1 Byte)
    let _b = true;
    let _b1: bool = false;
    let _c = 'C';
    let _c1: char = '💸';
}

fn compound_types() {
    println!("=========== COMPOUNT TYPES ============");
    //1. Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // Destructured Access

    let _five_hundred = tup.0; // Indexed Access
    let _six_point_four = tup.1;
    let _one = tup.2;
    println!("The value of y is: {y}");

    //2. Arrays
    let _a = [3; 5]; // [ 3, 3, 3, 3, 3 ]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index]; // Runtime length check - will error and exit if out of bounds
    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    // 3.1
    mutability();
    variables();
    shadowing();

    // 3.2
    types();
    compound_types();
}
