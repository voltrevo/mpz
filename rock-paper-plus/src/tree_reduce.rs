pub fn tree_reduce<T: Clone>(values: &[T], combine: &impl Fn(&T, &T) -> T) -> T {
    assert!(!values.is_empty());

    if values.len() == 1 {
        values[0].clone()
    } else {
        let mid = values.len() / 2;
        let (left, right) = values.split_at(mid);
        let left = tree_reduce(left, combine);
        let right = tree_reduce(right, combine);
        combine(&left, &right)
    }
}
