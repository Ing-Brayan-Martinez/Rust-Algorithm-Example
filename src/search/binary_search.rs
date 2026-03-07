
pub fn binary_search<T: Ord>(arr: &[T], objective: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let media = left + (right - left) / 2;

        match arr[media].cmp(objective) {
            std::cmp::Ordering::Equal => return Some(media),
            std::cmp::Ordering::Less => left = media + 1,
            std::cmp::Ordering::Greater => right = media,
        }
    }

    None
}