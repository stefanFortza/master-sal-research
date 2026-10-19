use rand::{Rng, RngExt};
use std::hint::black_box;

#[inline(never)]
pub fn constant_time_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a = black_box(a);
    let b = black_box(b);

    let mut accumulator: u8 = 0;

    for i in 0..32 {
        // XOR este 0 dacă octeții sunt identici, nenul dacă diferă.
        // OR acumulează orice diferență fără a se opri niciodată.
        accumulator |= black_box(a[i]) ^ black_box(b[i]);
    }

    // Întoarce true doar dacă accumulatorul a rămas strict 0.
    black_box(accumulator == 0)
}

#[inline(never)]
pub fn constant_time_compare_naive(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a = black_box(a);
    let b = black_box(b);

    for i in 0..32 {
        if black_box(a[i]) != black_box(b[i]) {
            return false;
        }
    }

    true
}

pub enum SampleClass {
    FullyEqual,     // matching == 32
    PartiallyEqual, // matching 0..31
}

pub fn generate_sample(class: SampleClass) -> ([u8; 32], [u8; 32]) {
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];

    let matching_count = match class {
        SampleClass::FullyEqual => 32,
        SampleClass::PartiallyEqual => rand::rng().random_range(0..32),
    };

    for j in 0..matching_count {
        a[j] = rand::rng().random::<u8>();
        b[j] = a[j];
    }

    for j in (matching_count + 1)..32 {
        a[j] = rand::rng().random::<u8>();
        b[j] = rand::rng().random::<u8>();
    }

    (a, b)
}
