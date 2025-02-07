
pub fn quick_sort(data: &mut [i32]) {
    let lower = 0;
    let upper = data.len() as isize - 1; // Use isize to handle potential negative bounds in recursive calls

    if upper > lower {
        if let Ok(partition_index) = partition(data, lower, upper as usize) { // Call partition with usize for upper bound
            internal_quick_sort(data, lower, (partition_index as isize - 1) as usize); // Adjust partition_index to isize for recursive calls
            internal_quick_sort(data, (partition_index as isize + 1) as usize as isize, upper as usize); // Adjust partition_index to isize for recursive calls
        }
    }
}

fn partition(data: &mut [i32], lower: isize, upper: usize) -> Result<usize, String> {
    if lower < 0 {
        return Err("Lower bound cannot be negative".to_string());
    }
    let lower_usize = lower as usize;

    let mut i = lower - 1;
    let pivot = data[upper];

    for j in lower_usize..upper {
        if data[j] <= pivot {
            i += 1;
            data.swap(i as usize, j);
        }
    }

    data.swap((i + 1) as usize, upper);
    Ok((i + 1) as usize)
}

fn internal_quick_sort(data: &mut [i32], lower: isize, upper: usize) {
    if upper as isize > lower {
        if let Ok(partition_index) = partition(data, lower, upper) {
            internal_quick_sort(data, lower, (partition_index as isize - 1) as usize);
            internal_quick_sort(data, (partition_index as isize + 1) as usize as isize, upper);
        }
    }
}