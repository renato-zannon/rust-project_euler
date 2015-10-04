/* Problem 3: Largest prime factor
 *
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ? */

fn main() {
    let factors = Factors { remaining: 600851475143u64, divisor: 2 };
    println!("{}", factors.max().unwrap());
}

struct Factors {
    remaining: u64,
    divisor: u64,
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.remaining <= 1 {
            return None;
        }

        while self.remaining % self.divisor > 0 {
            self.divisor += 1;
        }

        self.remaining /= self.divisor;
        Some(self.divisor)
    }
}
