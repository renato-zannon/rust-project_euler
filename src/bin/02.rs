/* Problem 2: Even Fibonacci numbers
 *
 * Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
 * starting with 1 and 2, the first 10 terms will be:
 *
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 *
 * By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
 * the sum of the even-valued terms. */

fn main() {
    let result = fibs()
        .take_while(|&n| n < 4_000_000)
        .filter(|&num| num % 2 == 0)
        .fold(0, |acc, num| acc + num);

    println!("{}", result);
}

fn fibs() -> Fibonacci {
    Fibonacci {
        pprev: None,
        prev: None,
    }
}

struct Fibonacci {
    prev: Option<u64>,
    pprev: Option<u64>,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let pprev = self.pprev;
        let prev = self.prev;

        if pprev.is_none() {
            self.pprev = Some(1);
            Some(1)
        } else if prev.is_none() {
            self.prev = Some(2);
            Some(2)
        } else {
            let current = Some(prev.unwrap() + pprev.unwrap());

            self.pprev = prev;
            self.prev = current;

            current
        }
    }
}
