pub fn get_data() -> Vec<i32> {
    vec![6, 2, 26, 12, 1, 5, 8, 14, 10, 17, 32]
}

pub fn print_data(data: &[i32]) {
    for element in data.iter() {
        println!("{}", element);
    }
}