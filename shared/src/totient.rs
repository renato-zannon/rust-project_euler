use sieve::{self, Sieve};

pub fn up_to(n: u64) -> Vec<u64> {
    let mut prime_sieve: Sieve<u64> = sieve::new();
    prime_sieve.compute_until(n);
    let primes = prime_sieve.found_primes();
    let mut phis: Vec<_> = (1..=n).map(|n| n as f64).collect();

    for &prime in primes {
        let prime_f = prime as f64;

        for phi in phis.iter_mut().step_by(prime as usize) {
            *phi *= (prime_f - 1.0) / prime_f;
        }
    }

    phis.into_iter().skip(2).map(|phi| phi as u64).collect()
}
