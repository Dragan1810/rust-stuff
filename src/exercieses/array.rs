pub fn truncate_vector(arr: &mut Vec<i32>, num: usize) {
    arr.truncate(num);
}

pub fn as_slice_vector(arr: &Vec<i32>) -> &[i32] {
    arr.as_slice()
}

pub fn swap_remove_vector<T>(arr: &mut Vec<T>, index: usize) -> T {
    arr.swap_remove(index)
}

pub fn insert_vector<T>(arr: &mut Vec<T>, index: usize, element: T) {
    arr.insert(index, element)
}
