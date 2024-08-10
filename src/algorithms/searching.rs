use crate::data_structures::array::Array;  // Import Array from the ds folder

pub fn binary_search(array: &Array, target: i32) -> Option<usize> {
    let data = &array.data;
    let mut low = 0;
    let mut high = data.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if data[mid] == target {
            return Some(mid);
        } else if data[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

// TODO: Implement other search algorithms (e.g., Linear Search, Depth-First Search).