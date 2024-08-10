use crate::data_structures::array::Array;  // Import Array from the ds folder

pub fn bubble_sort(array: &mut Array) {
    let n = array.data.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if array.data[j] > array.data[j + 1] {
                array.data.swap(j, j + 1);
            }
        }
    }
    // TODO: Implement step-by-step visualization support.
}

// TODO: Implement Quick Sort algorithm.
// TODO: Implement Merge Sort algorithm.