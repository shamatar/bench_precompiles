pub fn measure<F: Fn() -> Result<(), ()>>(runnable: &F, num_attempts: usize) -> u128 {
    use std::time::Instant;
    
    let mut total = 0u128;
    for _ in 0..num_attempts {
        let start = Instant::now();
        let r = runnable();
        let elapsed_nanos = start.elapsed().as_nanos();
        assert!(r.is_ok());
        total += elapsed_nanos;
    }

    total
} 