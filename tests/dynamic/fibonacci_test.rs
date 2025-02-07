use rust_algorithm_example::dynamic::fibonacci::fibonacci;

#[test]
fn fibonacci_test() {
    let num = 20;
    let expected_value = 6765;

    println!("-- Fibonacci -- \n\n");

    let result_value = fibonacci(num);

    println!("Fibonacci of {} is {} \n\n", num, result_value);

    assert_eq!(expected_value, result_value, "Test Failed: Fibonacci of {} should be {}", num, expected_value);

    println!("Test Passed!");
}