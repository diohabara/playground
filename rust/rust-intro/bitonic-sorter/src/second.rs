use std::cmp::Ordering;
use std::f64;
use SortOrder::*;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use crate::SortOrder::*;

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        sort(&mut x, true);
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        sort(&mut x, true);
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_f64() {
        let mut x = vec![20.0, -30.0, 11.0, 10.0];
        sort(&mut x, true);
    }
}
