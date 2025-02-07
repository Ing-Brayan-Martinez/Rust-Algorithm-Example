use rust_algorithm_example::dynamic::factorial::factorial;

#[test]
fn factorial_test() {
    let num = 5;
    let expected_value = 120;

    println!("-- Factorial -- \n\n");

    let result_value = factorial(num);

    println!("Factorial of {} is {} \n\n", num, result_value);

    assert_eq!(expected_value, result_value, "Test Failed: Factorial of {} should be {}", num, expected_value);

    println!("Test Passed!");
}