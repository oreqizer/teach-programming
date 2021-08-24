fn main() {
    // Comment this after changing i32 to f32:
    println!("div({}, {}) = {}", 5, 2, div(5, 2));

    // Uncomment this after changing i32 to f32:
    // println!("div({}, {}) = {}", 5.0, 2.0, div(5.0, 2.0));
}

// Change i32 to f32:
fn div(a: i32, b: i32) -> i32 {
    a / b
}
