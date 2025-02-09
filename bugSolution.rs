fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    println!("x = {}", z);

    *y = 10; // Modify the value through y

    //To fix the issue we remove the immutable reference z before making changes with y
    println!("x = {}", x);
} 