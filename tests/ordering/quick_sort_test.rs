use rust_algoritm_example::ordering::quick_sort::quick_sort;
use rust_algoritm_example::util::data::get_data;
use rust_algoritm_example::util::data::print_data;

#[test]
fn quick_sort_test() {
    let mut data = get_data();

    println!("-- QuickSort -- \n\n");

    //before
    println!("Before Sorting: ");
    print_data(&data);

    //sort
    println!("\n");
    quick_sort(&mut data);

    //after
    println!("After Sorting: ");
    print_data(&data);

    println!("\nTest Passed!");
}