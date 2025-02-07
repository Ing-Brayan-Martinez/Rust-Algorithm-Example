use rust_algorithm_example::ordering::selection_sort::selection_sort;
use rust_algorithm_example::util::data::get_data;
use rust_algorithm_example::util::data::print_data;

#[test]
fn selection_sort_test() {
    let mut data = get_data();

    println!("-- SelectionSort -- \n\n");

    //before
    println!("Before Sorting: ");
    print_data(&data);

    //sort
    println!("\n");
    selection_sort(&mut data);

    //after
    println!("After Sorting: ");
    print_data(&data);

    println!("\nTest Passed!");
}