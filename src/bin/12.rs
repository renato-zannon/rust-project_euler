/* Problem 12: Highly divisible triangular number
 *
 * The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle
 * number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
 *
 * 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 *
 * Let us list the factors of the first seven triangle numbers:
 *
 *    1: 1
 *    3: 1,3
 *    6: 1,2,3,6
 *   10: 1,2,5,10
 *   15: 1,3,5,15
 *   21: 1,3,7,21
 *   28: 1,2,4,7,14,28
 *
 * We can see that 28 is the first triangle number to have over five divisors.
 *
 * What is the value of the first triangle number to have over five hundred divisors? */

const NUMBER_OF_DIVISORS: uint = 500;

fn main() {
    let result = triangular_numbers::new().find(|&num| {
        let divisor_count = range(1, (num as f64).sqrt() as u64).fold(0, |sum, candidate| {
            if num % candidate == 0 {
                sum + 2
            } else {
                sum
            }
        });

        divisor_count > NUMBER_OF_DIVISORS
    });

    println!("{}", result);
}

mod triangular_numbers {
    use std::iter::count;

    pub struct TriangularNumbers<'a> {
        iter: Box<Iterator<u64> + 'a>
    }

    impl<'a> Iterator<u64> for TriangularNumbers<'a> {
        fn next(&mut self) -> Option<u64> {
            self.iter.next()
        }
    }

    pub fn new<'a>() -> TriangularNumbers<'a> {
        let iter = count(1u64, 1).scan(0u64, |sum, number| {
            *sum = *sum + number;
            Some(*sum)
        });

        TriangularNumbers { iter: box iter }
    }
}