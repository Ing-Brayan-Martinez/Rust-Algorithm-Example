use rust_algorithm_example::ordering::bubble_sort::bubble_sort;
use rust_algorithm_example::util::data::get_data;
use rust_algorithm_example::util::data::print_data;

#[test]
fn bubble_sort_test() {
    let mut data = get_data(); // get_data() needs to return a mutable Vec in Rust

    println!("-- BubbleSort -- \n\n");

    //before
    println!("Before Sorting: ");
    print_data(&data); // pass a reference to data

    //sort
    println!("\n");
    bubble_sort(&mut data); // bubble_sort expects a mutable slice

    //after
    println!("After Sorting: ");
    print_data(&data); // pass a reference to data

    println!("\nTest Passed!");
}
