#[cfg(target_arch = "aarch64")]
use std::arch::asm;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
use std::borrow::Borrow;


#[cfg(target_arch = "x86_64")]
const PREFETCH_OFFSET: usize = 1 + 8;
#[cfg(target_arch = "aarch64")]
const PREFETCH_OFFSET: usize = 1;


#[inline(always)]
unsafe fn prefetch(ptr: *const i8,) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        _mm_prefetch(ptr, _MM_HINT_T0,);
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("prfm pldl1keep, [{}]", in(reg) ptr);
    }
}


#[must_use]
pub fn create_layout_eytzinger<T: Clone,>(data: &[T],) -> Vec<T,> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }

    let mut eytzinger_data = vec![data[0].clone(); n];
    fn aux<T: Clone,>(source: &[T], dest: &mut [T], k: usize, i: &mut usize,) {
        if k >= dest.len() {
            return;
        }

        aux(source, dest, 2 * k + 1, i,);
        dest[k] = source[*i].clone();
        *i += 1;
        aux(source, dest, 2 * k + 2, i,);
    }

    aux(data, &mut eytzinger_data, 0, &mut 0,);
    eytzinger_data
}


#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn lower_bound<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        if data[cur as usize].borrow() >= target {
            res = Some(cur,);
            cur = 2 * cur + 1;
        } else {
            cur = 2 * cur + 2;
        }
    }

    res
}

#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn upper_bound<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        if data[cur as usize].borrow() > target {
            res = Some(cur,);
            cur = 2 * cur + 1;
        } else {
            cur = 2 * cur + 2;
        }
    }

    res
}


#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn lower_bound_prefetched<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let base_ptr = data.as_ptr();
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        unsafe {
            let lookahead_idx = (2 * cur + PREFETCH_OFFSET) * 4;
            if lookahead_idx < n {
                let ptr = base_ptr.add(lookahead_idx,) as *const i8;
                prefetch(ptr,);
            }
        }

        if data[cur as usize].borrow() >= target {
            res = Some(cur,);
            cur = 2 * cur + 1;
        } else {
            cur = 2 * cur + 2;
        }
    }

    res
}

#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn upper_bound_prefetched<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let base_ptr = data.as_ptr();
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        unsafe {
            let lookahead_idx = (2 * cur + PREFETCH_OFFSET) * 4;
            if lookahead_idx < n {
                let ptr = base_ptr.add(lookahead_idx,) as *const i8;
                prefetch(ptr,);
            }
        }

        if data[cur as usize].borrow() > target {
            res = Some(cur,);
            cur = 2 * cur + 1;
        } else {
            cur = 2 * cur + 2;
        }
    }

    res
}


#[must_use]
#[inline(always)]
pub fn lower_bound_prefetched_branchless<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let base_ptr = data.as_ptr();
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        unsafe {
            let lookahead_idx = (2 * cur + PREFETCH_OFFSET) * 4;
            if lookahead_idx < n {
                let ptr = base_ptr.add(lookahead_idx,) as *const i8;
                prefetch(ptr,);
            }
        }

        let right = (data[cur].borrow() < target) as usize;

        if right == 0 {
            res = Some(cur,);
        }

        cur = 2 * cur + 1 + right
    }

    res
}

#[expect(unused)]
#[must_use]
#[inline(always)]
pub fn upper_bound_prefetched_branchless<B, T,>(data: &[B], target: &T,) -> Option<usize,>
where
    B: Borrow<T,>,
    T: PartialOrd + ?Sized,
{
    let base_ptr = data.as_ptr();
    let n = data.len();
    if n == 0 {
        return None;
    }

    let mut res: Option<usize,> = None;
    let mut cur = 0;

    while cur < n {
        unsafe {
            let lookahead_idx = (2 * cur + PREFETCH_OFFSET) * 4;
            if lookahead_idx < n {
                let ptr = base_ptr.add(lookahead_idx,) as *const i8;
                prefetch(ptr,);
            }
        }

        let right = (data[cur].borrow() <= target) as usize;

        if right == 0 {
            res = Some(cur,);
        }

        cur = 2 * cur + 1 + right
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eytzinger_layout_structure() {
        // Input: Sorted array [10, 20, 30, 40, 50, 60, 70]
        // Tree View:
        //       40
        //     /    \
        //   20      60
        //  /  \    /  \
        // 10  30  50  70
        //
        // BFS Order (Array): [40, 20, 60, 10, 30, 50, 70]

        let input = vec![10, 20, 30, 40, 50, 60, 70];
        let layout = create_layout_eytzinger(&input,);

        assert_eq!(layout, vec![40, 20, 60, 10, 30, 50, 70]);
    }

    #[test]
    fn test_eytzinger_layout_imperfect_tree() {
        // Input: [1, 2, 3, 4, 5] (Not a perfect power of 2)
        // Tree:
        //      4
        //    /   \
        //   2     5
        //  / \
        // 1   3
        //
        // BFS: [4, 2, 5, 1, 3]

        let input = vec![1, 2, 3, 4, 5];
        let layout = create_layout_eytzinger(&input,);

        assert_eq!(layout, vec![4, 2, 5, 1, 3]);
    }

    #[test]
    fn test_lower_bound_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First occurrence
        assert_eq!(data[lower_bound(&data, &20).unwrap()], 20);

        // 2. Target present once
        assert_eq!(data[lower_bound(&data, &30).unwrap()], 30);

        // 3. Target missing (insert in middle)
        // 25 is > 20 but < 30. Should return index of 30 (idx 4).
        assert_eq!(data[lower_bound(&data, &25).unwrap()], 30);

        // 4. Target smaller than all (insert at start)
        assert_eq!(data[lower_bound(&data, &5).unwrap()], 10);

        // 5. Target larger than all (insert at end)
        assert_eq!(lower_bound(&data, &50), None);
    }

    #[test]
    fn test_upper_bound_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First element STRICTLY greater
        assert_eq!(data[upper_bound(&data, &20).unwrap()], 30);

        // 2. Target present once
        assert_eq!(data[upper_bound(&data, &30).unwrap()], 40);

        // 3. Target missing (behavior identical to lower_bound for gaps)
        assert_eq!(data[upper_bound(&data, &25).unwrap()], 30);

        // 5. Target larger than all (insert at end)
        assert_eq!(upper_bound(&data, &50), None);
    }

    #[test]
    fn test_lower_bound_prefetched_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First occurrence
        assert_eq!(data[lower_bound_prefetched(&data, &20).unwrap()], 20);

        // 2. Target present once
        assert_eq!(data[lower_bound_prefetched(&data, &30).unwrap()], 30);

        // 3. Target missing (insert in middle)
        // 25 is > 20 but < 30. Should return index of 30 (idx 4).
        assert_eq!(data[lower_bound_prefetched(&data, &25).unwrap()], 30);

        // 4. Target smaller than all (insert at start)
        assert_eq!(data[lower_bound_prefetched(&data, &5).unwrap()], 10);

        // 5. Target larger than all (insert at end)
        assert_eq!(lower_bound_prefetched(&data, &50), None);
    }

    #[test]
    fn test_upper_bound_prefetched_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First element STRICTLY greater
        assert_eq!(data[upper_bound_prefetched(&data, &20).unwrap()], 30);

        // 2. Target present once
        assert_eq!(data[upper_bound_prefetched(&data, &30).unwrap()], 40);

        // 3. Target missing (behavior identical to lower_bound for gaps)
        assert_eq!(data[upper_bound_prefetched(&data, &25).unwrap()], 30);

        // 5. Target larger than all (insert at end)
        assert_eq!(upper_bound_prefetched(&data, &50), None);
    }

    #[test]
    fn test_lower_bound_prefetched_branchless_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First occurrence
        assert_eq!(data[lower_bound_prefetched_branchless(&data, &20).unwrap()], 20);

        // 2. Target present once
        assert_eq!(data[lower_bound_prefetched_branchless(&data, &30).unwrap()], 30);

        // 3. Target missing (insert in middle)
        // 25 is > 20 but < 30. Should return index of 30 (idx 4).
        assert_eq!(data[lower_bound_prefetched_branchless(&data, &25).unwrap()], 30);

        // 4. Target smaller than all (insert at start)
        assert_eq!(data[lower_bound_prefetched_branchless(&data, &5).unwrap()], 10);

        // 5. Target larger than all (insert at end)
        assert_eq!(lower_bound_prefetched_branchless(&data, &50), None);
    }

    #[test]
    fn test_upper_bound_prefetched_branchless_correctness() {
        let data = vec![10, 20, 20, 20, 30, 40];
        let data = create_layout_eytzinger(&data,);

        // 1. Target present multiple times -> First element STRICTLY greater
        assert_eq!(data[upper_bound_prefetched_branchless(&data, &20).unwrap()], 30);

        // 2. Target present once
        assert_eq!(data[upper_bound_prefetched_branchless(&data, &30).unwrap()], 40);

        // 3. Target missing (behavior identical to lower_bound for gaps)
        assert_eq!(data[upper_bound_prefetched_branchless(&data, &25).unwrap()], 30);

        // 5. Target larger than all (insert at end)
        assert_eq!(upper_bound_prefetched_branchless(&data, &50), None);
    }
}
