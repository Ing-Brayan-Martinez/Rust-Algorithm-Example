pub fn fizzbuzz(n: i32) -> String {
    match n {
        _ if n % 3 == 0 && n % 5 == 0 => "FizzBuzz".to_string(),
        _ if n % 3 == 0 => "Fizz".to_string(),
        _ if n % 5 == 0 => "Buzz".to_string(),
        _ => n.to_string(),
    }
}