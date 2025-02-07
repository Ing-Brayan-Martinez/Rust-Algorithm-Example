
pub fn selection_sort(data: &mut [i32]) {
    for i in 0..(data.len() - 1) {
        let mut min_index = i;
        for j in (i + 1)..data.len() {
            if data[min_index] > data[j] {
                min_index = j;
            }
        }
        if min_index != i {
            data.swap(i, min_index); // Use Rust's built-in slice::swap for correct and efficient swapping
        }
    }
}