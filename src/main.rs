mod exercieses;

fn main() {
    let mut test_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    exercieses::array::truncate_vector(&mut test_vec, 3);
    let _as_slice = exercieses::array::as_slice_vector(&test_vec);
    let _swaped = exercieses::array::swap_remove_vector(&mut test_vec, 1);
    exercieses::array::insert_vector(&mut test_vec, 0, 8);
    dbg!(test_vec);
}
