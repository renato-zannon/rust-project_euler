/* Problem 8: Largest product in a series
 *
 * Find the greatest product of five consecutive digits in the 1000-digit number. */

use std::char::to_digit;
use std::iter::MultiplicativeIterator;

const NUMBER: &'static str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

const DIGIT_COUNT: uint = 5;

fn main() {
    let (string, result) = find_biggest(consecutive_digits::new(NUMBER, DIGIT_COUNT));
    println!("digits: {}\nproduct: {}", string, result);
}

fn find_biggest(slices: consecutive_digits::ConsecutiveDigits) -> (&str, uint) {
    slices.map(|str| (str, multiply(str)))
        .max_by(|&(_, num)| num)
        .unwrap()
}

fn multiply(num: &str) -> uint {
    num.chars().map(|char| to_digit(char, 10).unwrap()).product()
}

mod consecutive_digits {
    pub struct ConsecutiveDigits<'a> {
        string: &'a str,
        current: uint,
        length: uint,
    }

    impl<'a> Iterator<&'a str> for ConsecutiveDigits<'a> {
        fn next(&mut self) -> Option<&'a str> {
            let start = self.current;
            let end   = start + self.length;

            if end > self.string.len() - 1 {
                None
            } else {
                self.current = start + 1;
                Some(self.string.slice(start, end))
            }
        }
    }

    pub fn new(string: &str, length: uint) -> ConsecutiveDigits {
        ConsecutiveDigits {
            string: string,
            length: length,
            current: 0,
        }
    }
}
