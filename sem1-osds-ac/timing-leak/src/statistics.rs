#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WelfordAccumulator {
    count: u64,
    mean: f64,
    m2: f64,
}

impl Default for WelfordAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl WelfordAccumulator {
    #[inline]
    pub const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    #[inline]
    pub fn update(&mut self, sample: f64) {
        self.count += 1;
        let delta = sample - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = sample - self.mean;
        self.m2 += delta * delta2;
    }

    #[inline]
    pub const fn count(&self) -> u64 {
        self.count
    }

    #[inline]
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    #[inline]
    pub fn sample_variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / ((self.count - 1) as f64)
        }
    }

    #[inline]
    pub fn variance_of_mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sample_variance() / (self.count as f64)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WelchTestResult {
    pub t_value: f64,
    pub degrees_of_freedom: f64,
    pub count_a: u64,
    pub count_b: u64,
}

impl WelchTestResult {
    /// Pragul standard folosit în evaluarea constant-time (|t| > 4.5 indică leak)
    pub const LEAK_THRESHOLD: f64 = 4.5;

    #[inline]
    pub fn has_leak(&self) -> bool {
        self.t_value.abs() > Self::LEAK_THRESHOLD
    }
}

/// Calculează testul Welch între clasa A (de ex. intrări fixe/valide) și clasa B (intrări aleatoare).
pub fn calculate_welch_t(
    group_a: &WelfordAccumulator,
    group_b: &WelfordAccumulator,
) -> Option<WelchTestResult> {
    if group_a.count() < 2 || group_b.count() < 2 {
        return None;
    }

    let var_a_mean = group_a.variance_of_mean();
    let var_b_mean = group_b.variance_of_mean();
    let denom = (var_a_mean + var_b_mean).sqrt();

    // Dacă ambele seturi au varianță 0, nu există diferență
    if denom == 0.0 || denom.is_nan() {
        return Some(WelchTestResult {
            t_value: 0.0,
            degrees_of_freedom: 0.0,
            count_a: group_a.count(),
            count_b: group_b.count(),
        });
    }

    let t = (group_a.mean() - group_b.mean()) / denom;

    // Formula Welch–Satterthwaite pentru gradele de libertate
    let num_df = (var_a_mean + var_b_mean).powi(2);
    let den_df = (var_a_mean.powi(2) / ((group_a.count() - 1) as f64))
        + (var_b_mean.powi(2) / ((group_b.count() - 1) as f64));

    let df = if den_df > 0.0 { num_df / den_df } else { 0.0 };

    Some(WelchTestResult {
        t_value: t,
        degrees_of_freedom: df,
        count_a: group_a.count(),
        count_b: group_b.count(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welford_accuracy() {
        let mut acc = WelfordAccumulator::new();
        let samples = [10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0];
        for &s in &samples {
            acc.update(s);
        }

        assert_eq!(acc.count(), 8);
        assert!((acc.mean() - 18.0).abs() < 1e-9);
        // Varianța eșantionului: 28.57142857...
        assert!((acc.sample_variance() - 28.57142857142857).abs() < 1e-6);
    }

    #[test]
    fn test_welch_leak_detection() {
        let mut group_a = WelfordAccumulator::new();
        let mut group_b = WelfordAccumulator::new();

        // Două grupuri cu diferență mică de timp (simulare branch predict leak)
        for _ in 0..10_000 {
            group_a.update(100.0);
            group_b.update(105.0);
        }

        let res = calculate_welch_t(&group_a, &group_b).unwrap();
        // Deoarece varianța este practic nulă și media diferă, t-value explodează
        assert!(res.has_leak());
    }
}
