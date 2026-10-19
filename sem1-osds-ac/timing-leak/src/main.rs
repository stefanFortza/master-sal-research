mod cycle_counter;
mod statistics;
mod victims;

use std::{env, process};

use cycle_counter::measure_cycles;

use crate::victims::{constant_time_compare, constant_time_compare_naive, generate_sample};

fn run_benchmark_for_victim<F>(name: &str, sample_count: usize, mut compare_fn: F)
where
    F: FnMut(&[u8; 32], &[u8; 32]) -> bool,
{
    println!("\n=== Testing: {name} ({sample_count} samples) ===");

    let mut acc_fixed = statistics::WelfordAccumulator::new();
    let mut acc_random = statistics::WelfordAccumulator::new();

    for _ in 0..sample_count {
        let class_bit = rand::random::<bool>();

        let (a, b) = match class_bit {
            true => generate_sample(victims::SampleClass::FullyEqual),
            false => generate_sample(victims::SampleClass::PartiallyEqual),
        };

        let time = unsafe {
            measure_cycles(|| {
                compare_fn(&a, &b);
            })
        };

        if time < 2000 {
            if class_bit {
                acc_fixed.update(time as f64);
            } else {
                acc_random.update(time as f64);
            }
        }
    }

    let result = statistics::calculate_welch_t(&acc_fixed, &acc_random).unwrap();

    println!(
        "Group Fixed:  count = {}, mean = {:.2}, var = {:.2}",
        acc_fixed.count(),
        acc_fixed.mean(),
        acc_fixed.sample_variance()
    );
    println!(
        "Group Random: count = {}, mean = {:.2}, var = {:.2}",
        acc_random.count(),
        acc_random.mean(),
        acc_random.sample_variance()
    );
    println!(
        "Welch t-score: {:.2} (df = {:.2})",
        result.t_value, result.degrees_of_freedom
    );

    if result.has_leak() {
        println!("❌ TIMING LEAK DETECTED (|t| > 4.5)");
    } else {
        println!("✅ OK: CONSTANT TIME (|t| <= 4.5)");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sample_count = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(200_000);

    // Rulează pe rând ambele funcții
    run_benchmark_for_victim(
        "constant_time_compare_naive",
        sample_count,
        constant_time_compare_naive,
    );
    run_benchmark_for_victim("constant_time_compare", sample_count, constant_time_compare);
}
