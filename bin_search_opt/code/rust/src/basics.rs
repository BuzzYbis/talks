use std::borrow::Borrow;


#[must_use]
#[inline(always)]
pub fn lower_bound<B, T,>(data: &[B], target: &T,) -> usize
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let mut left: i32 = -1;
    let mut right = data.len() as i32;

    while right > left + 1 {
        let mid = left + (right - left) / 2;

        if data[mid as usize].borrow() >= target {
            right = mid;
        } else {
            left = mid;
        }
    }

    right as usize
}

#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn upper_bound<B, T,>(data: &[B], target: &T,) -> usize
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let mut left: i32 = -1;
    let mut right = data.len() as i32;

    while right > left + 1 {
        let mid = left + (right - left) / 2;

        if data[mid as usize].borrow() > target {
            right = mid;
        } else {
            left = mid;
        }
    }

    right as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound_correctness() {
        // Index: 0   1   2   3   4   5
        // Data: 10, 20, 20, 20, 30, 40
        let data = vec![10, 20, 20, 20, 30, 40];

        // 1. Target present multiple times -> First occurrence
        assert_eq!(lower_bound(&data, &20), 1);

        // 2. Target present once
        assert_eq!(lower_bound(&data, &30), 4);

        // 3. Target missing (insert in middle)
        // 25 is > 20 but < 30. Should return index of 30 (idx 4).
        assert_eq!(lower_bound(&data, &25), 4);

        // 4. Target smaller than all (insert at start)
        assert_eq!(lower_bound(&data, &5), 0);

        // 5. Target larger than all (insert at end)
        assert_eq!(lower_bound(&data, &50), 6);
    }

    #[test]
    fn test_upper_bound_correctness() {
        // Index: 0   1   2   3   4   5
        // Data: 10, 20, 20, 20, 30, 40
        let data = vec![10, 20, 20, 20, 30, 40];

        // 1. Target present multiple times -> First element STRICTLY greater
        // Elements > 20 starts at 30 (idx 4)
        assert_eq!(upper_bound(&data, &20), 4);

        // 2. Target present once
        // Element > 30 is 40 (idx 5)
        assert_eq!(upper_bound(&data, &30), 5);

        // 3. Target missing (behavior identical to lower_bound for gaps)
        assert_eq!(upper_bound(&data, &25), 4);

        // 4. Target larger than all
        assert_eq!(upper_bound(&data, &50), 6);
    }

    #[test]
    fn test_empty_array() {
        let empty: Vec<i32,> = vec![];
        assert_eq!(lower_bound(&empty, &10), 0);
        assert_eq!(upper_bound(&empty, &10), 0);
    }
}
