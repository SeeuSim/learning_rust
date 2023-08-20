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

fn main() {
    mutability();

    variables();

    shadowing();
}
