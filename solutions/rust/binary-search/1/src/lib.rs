pub fn find(array: &[i32], key: i32) -> Option<usize> {
    /*todo!(
        "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
    );*/
    let mut low = 0;
    let mut high = array.len();
    while low < high {
        let mid = (low + high) / 2;
        match array[mid] {
            x if x == key => return Some(mid),
            x if x < key => low = mid + 1,
            _ => high = mid,
        }
    }
    None
}
