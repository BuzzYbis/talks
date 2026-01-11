#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m256i,
    _mm256_castsi256_ps,
    _mm256_cmpgt_epi32,
    _mm256_loadu_si256,
    _mm256_movemask_ps,
    _mm256_set1_epi32,
    _mm_prefetch,
    _MM_HINT_T0,
};
#[cfg(target_arch = "aarch64")]
use std::arch::{
    aarch64::{
        vaddq_s32,
        vaddvq_s32,
        vcgtq_s32,
        vdupq_n_s32,
        vld1q_s32,
        vreinterpretq_s32_u32,
    },
    asm,
};

const BLOCK_SIZE: usize = 16;
const B_PLUS_ONE: usize = BLOCK_SIZE + 1;
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

#[inline(always)]
unsafe fn simd_intrinsic_lower(ptr: *const i32, target: i32,) -> usize {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let t = _mm256_set1_epi32(target,);

        let v0 = _mm256_loadu_si256(ptr as *const __m256i,);
        let v1 = _mm256_loadu_si256(ptr.add(8,) as *const __m256i,);

        let m0 = _mm256_cmpgt_epi32(t, v0,);
        let m1 = _mm256_cmpgt_epi32(t, v1,);

        let b0 = _mm256_movemask_ps(_mm256_castsi256_ps(m0,),) as u32;
        let b1 = _mm256_movemask_ps(_mm256_castsi256_ps(m1,),) as u32;

        let cm = (b0 | (b1 << 8)).count_ones();

        cm as usize
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let t = vdupq_n_s32(target,);

        let v0 = vld1q_s32(ptr,);
        let v1 = vld1q_s32(ptr.add(4,),);
        let v2 = vld1q_s32(ptr.add(8,),);
        let v3 = vld1q_s32(ptr.add(12,),);

        let c0 = vcgtq_s32(t, v0,);
        let c1 = vcgtq_s32(t, v1,);
        let c2 = vcgtq_s32(t, v2,);
        let c3 = vcgtq_s32(t, v3,);

        let c0 = vreinterpretq_s32_u32(c0,);
        let c1 = vreinterpretq_s32_u32(c1,);
        let c2 = vreinterpretq_s32_u32(c2,);
        let c3 = vreinterpretq_s32_u32(c3,);

        let s0 = vaddq_s32(c0, c1,);
        let s1 = vaddq_s32(c2, c3,);
        let st = vaddq_s32(s0, s1,);

        let ss = vaddvq_s32(st,);

        (-ss) as usize
    }
}

#[inline(always)]
fn simd_intrinsic_upper(ptr: *const i32, target: i32,) -> usize {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let t = _mm256_set1_epi32(target,);

        let v0 = _mm256_loadu_si256(ptr as *const __m256i,);
        let v1 = _mm256_loadu_si256(ptr.add(8,) as *const __m256i,);

        let m0 = _mm256_cmpgt_epi32(v0, t,);
        let m1 = _mm256_cmpgt_epi32(v1, t,);

        let b0 = _mm256_movemask_ps(_mm256_castsi256_ps(m0,),) as u32;
        let b1 = _mm256_movemask_ps(_mm256_castsi256_ps(m1,),) as u32;

        let cm = (b0 | (b1 << 8)).count_ones() as usize;

        (16 - cm)
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let t = vdupq_n_s32(target,);

        let v0 = vld1q_s32(ptr,);
        let v1 = vld1q_s32(ptr.add(4,),);
        let v2 = vld1q_s32(ptr.add(8,),);
        let v3 = vld1q_s32(ptr.add(12,),);

        let c0 = vcgtq_s32(v0, t,);
        let c1 = vcgtq_s32(v1, t,);
        let c2 = vcgtq_s32(v2, t,);
        let c3 = vcgtq_s32(v3, t,);

        let c0 = vreinterpretq_s32_u32(c0,);
        let c1 = vreinterpretq_s32_u32(c1,);
        let c2 = vreinterpretq_s32_u32(c2,);
        let c3 = vreinterpretq_s32_u32(c3,);

        let s0 = vaddq_s32(c0, c1,);
        let s1 = vaddq_s32(c2, c3,);
        let st = vaddq_s32(s0, s1,);

        let ss = vaddvq_s32(st,);

        (16 + ss) as usize
    }
}


fn distribute_child_sizes(mut n: usize,) -> [usize; 17] {
    let mut sizes = [0; 17];

    if n <= BLOCK_SIZE {
        return sizes;
    }

    n -= BLOCK_SIZE;

    let mut level = 1;
    while n > 0 {
        let nodes_per_child = (B_PLUS_ONE as usize).pow(level - 1,);
        let level_capacity = BLOCK_SIZE * nodes_per_child;

        for i in 0..B_PLUS_ONE {
            if n == 0 {
                break;
            }
            let take = std::cmp::min(n, level_capacity,);
            sizes[i] += take;
            n -= take;
        }
        level += 1;
    }
    sizes
}


#[must_use]
pub fn create_layout_stree(data: &[i32],) -> Vec<i32,> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }

    let rem = n % BLOCK_SIZE;
    let pad = if rem == 0 { 0 } else { BLOCK_SIZE - rem };
    let mut stree = vec![i32::MAX; n + pad];

    fn aux(source: &[i32], dest: &mut [i32], idx: usize,) {
        let n = source.len();
        if n == 0 {
            return;
        }

        if n <= BLOCK_SIZE {
            for i in 0..n {
                dest[idx * BLOCK_SIZE + i] = source[i];
            }
            return;
        }

        let child_sizes = distribute_child_sizes(n,);

        let mut cur = 0;
        for i in 0..BLOCK_SIZE {
            let c_len = child_sizes[i];
            let pivot_val = source[cur + c_len];
            dest[idx * BLOCK_SIZE + i] = pivot_val;
            cur += c_len + 1;
        }

        let mut start = 0;
        for i in 0..B_PLUS_ONE {
            let c_len = child_sizes[i];
            if c_len > 0 {
                let child_node_idx = idx * B_PLUS_ONE + i + 1;
                if child_node_idx * BLOCK_SIZE < dest.len() {
                    aux(&source[start..start + c_len], dest, child_node_idx,);
                }
            }
            start += c_len + 1;
        }
    }

    aux(data, &mut stree, 0,);
    stree
}


macro_rules! impl_bound_stree {
    ($name:ident, $simd_intrinsic:ident) => {
        pub fn $name(data: &[i32], target: &i32,) -> Option<usize,> {
            let base_ptr = data.as_ptr();
            let n = data.len();
            if n == 0 {
                return None;
            }

            let mut res: Option<usize,> = None;
            let mut cur = 0;
            let t = *target;

            while (cur * BLOCK_SIZE) < n {
                let block_offset = cur * BLOCK_SIZE;
                let ptr = unsafe { base_ptr.add(block_offset,) };

                unsafe {
                    let lookahead_idx = (17 * cur + PREFETCH_OFFSET) * 16;
                    if lookahead_idx < n {
                        let ptr = base_ptr.add(lookahead_idx,) as *const i8;
                        prefetch(ptr,);
                    }
                }

                let i = unsafe { $simd_intrinsic(ptr, t,) };

                if i < 16 {
                    let candidate_val = unsafe { *ptr.add(i,) };
                    if candidate_val != i32::MAX {
                        res = Some(block_offset + i,);
                    }
                }

                cur = cur * 17 + i + 1;
            }

            res
        }
    };
}

impl_bound_stree!(lower_bound, simd_intrinsic_lower);
impl_bound_stree!(upper_bound, simd_intrinsic_upper);

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    // Helper to find "Truth" for lower_bound using std library
    fn std_lower_bound(data: &[i32], target: i32,) -> Option<usize,> {
        // lower_bound finds the first element where (x < target) is FALSE.
        // i.e., x >= target.
        let idx = data.partition_point(|&x| x < target,);
        if idx < data.len() { Some(idx,) } else { None }
    }

    // Helper to find "Truth" using std library
    fn std_upper_bound(data: &[i32], target: i32,) -> Option<usize,> {
        // partition_point returns the index of the first element satisfying the
        // predicate. For upper_bound, we want the first element where (x <=
        // target) is FALSE. i.e., x > target.
        let idx = data.partition_point(|&x| x <= target,);
        if idx < data.len() { Some(idx,) } else { None }
    }

    #[test]
    fn test_stree_layout_structure() {
        let input = vec![
            1, 2, 6, 7, 17, 19, 21, 24, 29, 33, 35, 38, 40, 45, 47, 49, 52, 54, 55, 61,
            62, 64, 75, 76, 77, 79, 81, 83, 84, 85, 88, 96,
        ];
        let layout = create_layout_stree(&input,);

        assert_eq!(
            layout,
            [
                52, 54, 55, 61, 62, 64, 75, 76, 77, 79, 81, 83, 84, 85, 88, 96, 1, 2, 6,
                7, 17, 19, 21, 24, 29, 33, 35, 38, 40, 45, 47, 49
            ]
        );

        let input = vec![
            1, 2, 6, 7, 17, 19, 21, 24, 29, 33, 35, 38, 40, 45, 47, 49, 52, 54, 55, 61,
            62, 64, 75, 76, 77, 79, 81,
        ];
        let layout = create_layout_stree(&input,);

        assert_eq!(
            layout,
            [
                38, 40, 45, 47, 49, 52, 54, 55, 61, 62, 64, 75, 76, 77, 79, 81, 1, 2, 6,
                7, 17, 19, 21, 24, 29, 33, 35, 2147483647, 2147483647, 2147483647,
                2147483647, 2147483647
            ]
        );
    }

    #[test]
    fn test_lower_bound_basic() {
        // Layout: [10, 20, 20, 20, 30]
        let data = vec![10, 20, 20, 20, 30];
        let layout = create_layout_stree(&data,);

        // Case 1: Target smaller than all (5) -> Returns 10
        // (First element >= 5 is 10)
        let res = lower_bound(&layout, &5,).unwrap();
        assert_eq!(layout[res], 10, "Target 5 should find 10");

        // Case 2: Target exact match (10) -> Returns 10
        let res = lower_bound(&layout, &10,).unwrap();
        assert_eq!(layout[res], 10, "Target 10 should find 10");

        // Case 3: Target duplicates (20) -> Returns 20
        // (It should find one of the 20s)
        let res = lower_bound(&layout, &20,).unwrap();
        assert_eq!(layout[res], 20, "Target 20 should find 20");

        // Case 4: Target in-between (25) -> Returns 30
        // (First element >= 25 is 30)
        let res = lower_bound(&layout, &25,).unwrap();
        assert_eq!(layout[res], 30, "Target 25 should find 30");

        // Case 5: Target larger than all (40) -> Returns None
        let res = lower_bound(&layout, &40,);
        assert!(res.is_none(), "Target 40 should return None");
    }

    #[test]
    fn test_lower_bound_fuzzy() {
        let mut rng = rand::thread_rng();

        // 1. Generate Random Data with duplicates
        let n = 2000;
        let mut data: Vec<i32,> = (0..n).map(|_| rng.gen_range(0..100,),).collect();
        data.sort_unstable(); // Must be sorted

        let layout = create_layout_stree(&data,);

        // 2. Test random targets
        for _ in 0..1000 {
            let target = rng.gen_range(0..110,);

            // Get Truth
            let truth_idx = std_lower_bound(&data, target,);
            let truth_val = truth_idx.map(|i| data[i],);

            // Get S-Tree Result
            let stree_idx = lower_bound(&layout, &target,);
            let stree_val = stree_idx.map(|i| layout[i],);

            assert_eq!(
                stree_val, truth_val,
                "Mismatch for target {}. Std: {:?}, Stree: {:?}",
                target, truth_val, stree_val
            );
        }
    }

    #[test]
    fn test_upper_bound_basic() {
        // Layout: [10, 20, 20, 20, 30]
        // Note: We need enough elements or padding to handle the block logic,
        // but create_layout_stree handles that.
        let data = vec![10, 20, 20, 20, 30];
        let layout = create_layout_stree(&data,);

        // Case 1: Target smaller than all (5) -> Returns 10
        let res = upper_bound(&layout, &5,).unwrap();
        assert_eq!(layout[res], 10, "Target 5 should find 10");

        // Case 2: Target exists (10) -> Returns 20 (Strictly greater)
        let res = upper_bound(&layout, &10,).unwrap();
        assert_eq!(layout[res], 20, "Target 10 should find 20");

        // Case 3: Target duplicates (20) -> Returns 30 (Skip all 20s)
        let res = upper_bound(&layout, &20,).unwrap();
        assert_eq!(layout[res], 30, "Target 20 should find 30");

        // Case 4: Target exists (30) -> Returns None (End of array)
        let res = upper_bound(&layout, &30,);
        assert!(res.is_none(), "Target 30 should return None");

        // Case 5: Target huge (100) -> Returns None
        let res = upper_bound(&layout, &100,);
        assert!(res.is_none(), "Target 100 should return None");
    }

    #[test]
    fn test_upper_bound_fuzzy() {
        let mut rng = rand::thread_rng();

        // 1. Generate Random Data with duplicates
        let n = 2000;
        let mut data: Vec<i32,> = (0..n).map(|_| rng.random_range(0..100,),).collect();
        data.sort_unstable(); // Must be sorted

        let layout = create_layout_stree(&data,);

        // 2. Test random targets
        for _ in 0..1000 {
            let target = rng.random_range(0..110,);

            // Get Truth
            let truth_idx = std_upper_bound(&data, target,);
            let truth_val = truth_idx.map(|i| data[i],);

            // Get S-Tree Result
            let stree_idx = upper_bound(&layout, &target,);
            let stree_val = stree_idx.map(|i| layout[i],);

            assert_eq!(
                stree_val, truth_val,
                "Mismatch for target {}. Std: {:?}, Stree: {:?}",
                target, truth_val, stree_val
            );
        }
    }
}
