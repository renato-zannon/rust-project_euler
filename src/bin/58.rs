/* Problem 58: Spiral primes
 *
 * Starting with 1 and spiralling anticlockwise in the following way, a square
 * spiral with side length 7 is formed.
 *
 * 37 36 35 34 33 32 31
 * 38 17 16 15 14 13 30
 * 39 18  5  4  3 12 29
 * 40 19  6  1  2 11 28
 * 41 20  7  8  9 10 27
 * 42 21 22 23 24 25 26
 * 43 44 45 46 47 48 49
 *
 * It is interesting to note that the odd squares lie along the bottom right diagonal, but what is
 * more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is, a
 * ratio of 8/13 ≈ 62%. If one complete new layer is wrapped around the spiral above, a square
 * spiral with side length 9 will be formed. If this process is continued, what is the side length
 * of the square spiral for which the ratio of primes along both diagonals first falls below 10%? */

extern crate shared;
use shared::primes;

#[cfg(not(test))]
fn main() {
    let mut square = Square::new();
    let mut diagonals_count: u32 = 5;

    while square.prime_count * 10 >= diagonals_count {
        square.grow();
        diagonals_count += 4;
    }

    println!("{}", square.size);
}

struct Square {
    size: u16,
    diagonal: [u32; 4],
    prime_count: u32,
}

impl Square {
    fn new() -> Square {
        let mut square = Square {
            size: 1,
            diagonal: [1, 1, 1, 1],
            prime_count: 0,
        };

        square.grow();
        square
    }

    fn grow(&mut self) {
        let increment = (self.size + 1) as u32;

        self.update_diagonal(0, 3, increment);
        self.update_diagonal(1, 0, increment);
        self.update_diagonal(2, 1, increment);
        self.update_diagonal(3, 2, increment);

        self.size += 2;
    }

    fn update_diagonal(&mut self, dest: usize, source: usize, increment: u32) {
        let value = self.diagonal[source] + increment;

        self.diagonal[dest] = value;

        if primes::is_prime(value) {
            self.prime_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Square;

    #[test]
    fn test_square_growth() {
        let mut square = Square::new();

        assert_eq!(&square.diagonal[..], &[3, 5, 7, 9][..]);
        assert_eq!(square.size, 3);
        assert_eq!(square.prime_count, 3);

        square.grow();
        assert_eq!(&square.diagonal[..], &[13, 17, 21, 25][..]);
        assert_eq!(square.size, 5);
        assert_eq!(square.prime_count, 5);

        square.grow();
        assert_eq!(&square.diagonal[..], &[31, 37, 43, 49][..]);
        assert_eq!(square.size, 7);
        assert_eq!(square.prime_count, 8);
    }
}
