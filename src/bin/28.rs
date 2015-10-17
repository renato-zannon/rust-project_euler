/* Problem 28: Number spiral diagonals
 *
 * Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is
 * formed as follows:
 *
 * 21 22 23 24 25
 * 20  7  8  9 10
 * 19  6  1  2 11
 * 18  5  4  3 12
 * 17 16 15 14 13
 *
 * It can be verified that the sum of the numbers on the diagonals is 101.
 *
 * What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way? */

#![feature(step_by)]

#[cfg(not(test))]
fn main() {
    let result: u32 = diagonals(1001).into_iter().fold(0, |acc, item| acc + item);
    println!("{}", result);
}

fn diagonals(square_size: u32) -> Vec<u32> {
    assert!(square_size % 2 == 1);

    let mut result  = vec!(1);
    let mut current = 1;

    for size in (3..square_size + 1).step_by(2) {
        for _ in 0..4 {
            current += size - 1;
            result.push(current);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::diagonals;

    #[test]
    fn test_width_1() {
        assert_eq!(diagonals(1), vec!(1));
    }

    #[test]
    fn test_width_3() {
        assert_eq!(diagonals(3), vec!(1, 3, 5, 7, 9));
    }

    #[test]
    fn test_width_5() {
        assert_eq!(diagonals(5), vec!(1, 3, 5, 7, 9, 13, 17, 21, 25));
    }

    #[test]
    fn test_width_7() {
        assert_eq!(diagonals(7), vec!(1, 3, 5, 7, 9, 13, 17, 21, 25, 31, 37, 43, 49));
    }
}
