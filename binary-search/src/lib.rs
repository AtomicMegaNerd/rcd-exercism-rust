use std::cmp::Ordering;

// Binary search
pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let arr = array.as_ref();
    let mid = arr.len() / 2;
    if let Some(mid_elem) = arr.get(mid) {
        let (left, right) = arr.split_at(mid);
        match key.cmp(mid_elem) {
            Ordering::Less => find(left, key),
            Ordering::Greater => find(&right[1..], key).map(|ix| ix + 1 + mid),
            Ordering::Equal => Some(mid),
        }
    } else {
        None
    }
}
