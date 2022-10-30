/// Box is a simple way to heap allocate data in Rust.
/// By default, values are stored on the stack.

/// Takes a number (by value) as an argument, and heap allocates it in a box.
fn box_the_number(number: usize) -> Box<usize> {
    Box::new(number)
}
