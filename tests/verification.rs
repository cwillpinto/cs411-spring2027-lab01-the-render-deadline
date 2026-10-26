use speedup::{
    amdahl_speedup, measure_sequential_fraction, parallel_sum_of_primes, sequential_sum_of_primes,
};

#[test]
fn parallel_matches_baseline_at_boundaries() {
    for limit in [0, 2, 3, 17, 101, 1_003] {
        let expected = sequential_sum_of_primes(limit);
        for workers in [1, 2, 4, 8] {
            assert_eq!(
                parallel_sum_of_primes(limit, workers),
                expected,
                "limit={limit}, workers={workers}"
            );
        }
    }
}

#[test]
fn amdahl_model_has_correct_edges() {
    assert!((amdahl_speedup(0.25, 1.0) - 1.0).abs() < 1e-12);
    assert!((amdahl_speedup(0.25, 8.0) - 2.909090909).abs() < 1e-8);
}

#[test]
fn measured_fraction_is_finite_and_bounded() {
    let s = measure_sequential_fraction(50_000);
    assert!(s.is_finite() && (0.0..=1.0).contains(&s), "s={s}");
}
