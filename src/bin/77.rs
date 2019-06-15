/* Problem 77: Prime summations
 *
 * It is possible to write ten as the sum of primes in exactly five different ways:
 *
 * 7 + 3
 * 5 + 5
 * 5 + 3 + 2
 * 3 + 3 + 2 + 2
 * 2 + 2 + 2 + 2 + 2
 *
 * What is the first value which can be written as the sum of primes in over five thousand different ways?
 **/

use shared::sieve;

const COMPUTE_UNTIL: i32 = 1000;
const TARGET: i32 = 5_000;

#[derive(Debug)]
struct WaysTracker {
    number: i32,
    ways: i32,
}
impl WaysTracker {
    fn track(&mut self, number: i32, ways: i32) {
        if ways < TARGET {
            return;
        }

        if self.ways < TARGET || number < self.number {
            *self = WaysTracker { number, ways };
        }
    }
}

fn main() {
    let sieve = sieve::new::<i32>();
    let primes = sieve.take_while(|&n| n <= COMPUTE_UNTIL);
    let mut sums = vec![0; COMPUTE_UNTIL as usize];
    sums[0] = 1;

    let mut ways_tracker = WaysTracker { number: 0, ways: 1 };

    for prime in primes {
        let prime = prime as usize;

        for index in prime..sums.len() {
            sums[index] += sums[index - prime];
            ways_tracker.track(index as i32, sums[index]);
        }
    }

    println!("{:?}", ways_tracker);
}
