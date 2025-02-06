use rust_algoritm_example::dynamic::fizzbuzz::fizzbuzz;

#[test]
fn fizzbuzz_test() {
    let num = 60;
    let expected_value = "FizzBuzz";

    println!("-- FizzBuzz -- \n\n");

    let result_value = fizzbuzz(num);

    println!("FizzBuzz of {} is {} \n\n", num, result_value);

    assert_eq!(expected_value, result_value, "\nTest Failed! FizzBuzz of {} should be '{}'", num, expected_value);

    println!("\nTest Passed!");
}