/* Problem 37: Truncatable primes
 *
 * The number 3797 has an interesting property. Being prime itself, it is possible to continuously
 * remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly
 * we can work from right to left: 3797, 379, 37, and 3.
 *
 * Find the sum of the only eleven primes that are both truncatable from left to right and right to
 * left.
 *
 * NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes. */


use shared::{digits, sieve};

fn main() {
    let mut primes = sieve::new::<u32>();
    let mut found = Vec::with_capacity(11);

    while found.len() < 11 {
        let prime = primes.next().unwrap();

        if prime > 10 && is_truncatable(prime, &mut primes) {
            found.push(prime);
        }
    }

    println!("{}", found.into_iter().fold(0, |acc, n| acc + n));
}

fn is_truncatable(prime: u32, sieve: &mut sieve::Sieve<u32>) -> bool {
    truncatable_from_right(prime, sieve) && truncatable_from_left(prime, sieve)
}

fn truncatable_from_left(prime: u32, sieve: &mut sieve::Sieve<u32>) -> bool {
    let prime_digits = digits::new::<_, u32>(prime);

    prime_digits
        .rev()
        .scan((0, 1), |state, digit| {
            let (previous, multiplier) = *state;
            let truncation = previous + digit * multiplier;

            *state = (truncation, multiplier * 10);
            Some(truncation)
        })
        .all(|truncation| sieve.is_prime(truncation))
}

fn truncatable_from_right(prime: u32, sieve: &mut sieve::Sieve<u32>) -> bool {
    let mut remaining = prime / 10;

    while remaining > 0 {
        if !sieve.is_prime(remaining) {
            return false;
        }

        remaining = remaining / 10;
    }

    return true;
}
