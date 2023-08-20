fn mutability() {
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

fn main() {
    mutability();

    variables();
}
