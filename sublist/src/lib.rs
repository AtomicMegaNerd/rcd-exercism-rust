use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> Comparison {
    let l1_len = l1.len();
    let l2_len = l2.len();

    match l1_len.cmp(&l2_len) {
        Ordering::Less => {
            if sublist_from_front(l1, l2) || sublist_from_back(l1, l2) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if sublist_from_front(l2, l1) || sublist_from_back(l2, l1) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Equal => match lists_are_equal(l1, l2) {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
    }
}

fn sublist_from_front<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    let mut sublist = l2;
    loop {
        sublist = &sublist[1..];
        if sublist.len() < l1.len() {
            break;
        }
        if lists_are_equal(l1, sublist) {
            return true;
        }
    }
    false
}

fn sublist_from_back<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    let mut sublist = l2;
    loop {
        sublist = &sublist[..sublist.len() - 1];
        if sublist.len() < l1.len() {
            break;
        }
        if lists_are_equal(l1, sublist) {
            return true;
        }
    }
    false
}

fn lists_are_equal<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    for (ix, item) in l1.iter().enumerate() {
        if item == &l2[ix] {
            continue;
        }
        return false;
    }
    true
}
