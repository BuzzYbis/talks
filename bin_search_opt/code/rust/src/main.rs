mod basics;
mod eytzinger;
mod stree;

use std::time::Instant;

use rand::{Rng, SeedableRng};

use crate::{eytzinger::create_layout_eytzinger, stree::create_layout_stree};


pub fn verify_integrity() {
    let mut rng = rand::rng();

    let n = 100_000;
    let mut data: Vec<i32,> = (0..n).map(|_| rng.random_range(0..i32::MAX,),).collect();
    data.sort_unstable();

    // 2. Build Layouts
    let eytz = create_layout_eytzinger(&data,);
    let stree = create_layout_stree(&data,);

    let queries = 10_000;
    for _ in 0..queries {
        let target = rng.random_range(0..i32::MAX,);

        let truth_idx = basics::lower_bound(&data, &target,);
        let truth_val =
            if truth_idx < data.len() { Some(data[truth_idx],) } else { None };

        let eytz_idx = eytzinger::lower_bound_prefetched_branchless(&eytz, &target,);
        let eytz_val = eytz_idx.map(|i| eytz[i],);

        let stree_idx = stree::lower_bound(&stree, &target,);
        let stree_val = stree_idx.map(|i| stree[i],);

        if eytz_val != truth_val {
            panic!(
                "INTEGRITY FAILURE: Eytzinger mismatch.\nTarget: {}\nExpected: {:?}\nGot: {:?}",
                target, truth_val, eytz_val
            );
        }

        if stree_val != truth_val {
            panic!(
                "INTEGRITY FAILURE: S-Tree mismatch.\nTarget: {}\nExpected: {:?}\nGot: {:?}",
                target, truth_val, stree_val
            );
        }
    }

    println!("Integrity Check Passed: All algorithms match.");
}


pub fn benchmark_performance(n: usize,) {
    let queries = 1_000_000;
    let size_mb = (n * 4) as f64 / 1_024.0 / 1_024.0;


    // Generate benchmark data
    println!("Generating benchmark data:");
    println!("  -> Generating vector of N = {} elements ({:.2} MB)", n, size_mb);
    println!("  -> Generating {} random queries", queries);

    let gen_start = Instant::now();

    // Generating target queries
    let mut rng = rand::prelude::StdRng::seed_from_u64(22,);
    let targets: Vec<i32,> =
        (0..queries).map(|_| rng.random_range(0..i32::MAX,),).collect();

    // Generating data for basic binary search
    let mut rng = rand::prelude::StdRng::seed_from_u64(222,);
    let mut data: Vec<i32,> = (0..n).map(|_| rng.random_range(0..i32::MAX,),).collect();
    data.sort_unstable();

    let mut checksum: usize = 0;

    println!("  -> Generation took: {:.2?}", gen_start.elapsed());


    println!("\nBenchmark basic lower bound:");

    let start = Instant::now();

    for target in &targets {
        checksum += basics::lower_bound(&data, target,);
    }

    let duration = start.elapsed();
    let ns_per_op = duration.as_nanos() as f64 / queries as f64;

    println!("  -> Total time: {:.2?}", duration);
    println!("  -> Latency:    \x1b[1;32m{:.2} ns/op\x1b[0m", ns_per_op);


    // // Ordering data for Eytzinger version
    // let data_eytzinger = create_layout_eytzinger(&data,);
    // drop(data,);
    //
    // println!("\nBenchmark Eytzinger prefetched branchless lower bound:");
    //
    // let start = Instant::now();
    //
    // for target in &targets {
    //     checksum +=
    //         eytzinger::lower_bound_prefetched_branchless(&data_eytzinger, target,)
    //             .unwrap_or(0,);
    // }
    //
    // let duration = start.elapsed();
    // let ns_per_op = duration.as_nanos() as f64 / queries as f64;
    //
    // println!("  -> Total time: {:.2?}", duration);
    // println!("  -> Latency:    \x1b[1;32m{:.2} ns/op\x1b[0m", ns_per_op);


    // Ordering data for stree version
    let data_stree = create_layout_stree(&data,);
    drop(data,);

    println!("\nBenchmark stree simd lower bound:");

    let start = Instant::now();

    for target in &targets {
        checksum += stree::lower_bound(&data_stree, target,).unwrap_or(0,);
    }

    let duration = start.elapsed();
    let ns_per_op = duration.as_nanos() as f64 / queries as f64;

    println!("  -> Total time: {:.2?}", duration);
    println!("  -> Latency:    \x1b[1;32m{:.2} ns/op\x1b[0m", ns_per_op);

    println!("\nChecksum {}", checksum);
}


fn main() {
    println!("========================= INTEGRITY ==========================");
    verify_integrity();

    println!("\n================== FOR 100 000 000 ELEMENTS ==================");
    benchmark_performance(100_000_000,);

    println!("\n================= FOR 1 000 000 000 ELEMENTS =================");
    benchmark_performance(1_000_000_000,);

    println!("\n================= FOR 1 500 000 000 ELEMENTS =================");
    benchmark_performance(1_500_000_000,);

    println!("\n================= FOR 2 000 000 000 ELEMENTS =================");
    benchmark_performance(2_000_000_000,);
}
