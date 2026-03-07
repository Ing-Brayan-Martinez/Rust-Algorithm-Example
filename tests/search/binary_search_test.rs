use rust_algorithm_example::search::binary_search::binary_search;

#[test]
fn selection_sort_test() {
    // Con enteros
    let nums = vec![10, 20, 30, 40, 50, 60];
    println!("{:?}", binary_search(&nums, &40));     // Some(3)

    // Con strings
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    println!("{:?}", binary_search(&words, &"cherry"));   // Some(2)

    println!("\nTest Passed!");
}