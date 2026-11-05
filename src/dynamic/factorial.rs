
pub fn factorial(n: i32) -> i32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}