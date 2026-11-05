
pub fn bubble_sort(data: &mut [i32]) {
    for i in 0..(data.len() - 1) {
        let mut swapped = false; // Use a boolean flag instead of a counter for clarity
        for j in 0..(data.len() - i - 1) {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1); // Use Rust's built-in slice::swap for efficiency and correctness
                swapped = true;
            }
        }
        if !swapped { // If no swaps occurred in this pass, the list is sorted
            break;
        }
    }
}
