fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    // This is fine since z is immutable
    println!("x = {}", z);

    *y = 10; // Modify the value through y

    // Compiler error: cannot borrow `x` as mutable because it is also borrowed as immutable
    // println!("x = {}", z); //This causes error
}