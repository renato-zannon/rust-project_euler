/* Problem 4: Largest palindrome product
 *
 * A palindromic number reads the same both ways. The largest palindrome made from the product of
 * two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers. */

struct Counter<'a> {
    values: &'a [u32],
    position: usize,
}

impl<'a> Iterator for Counter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let base = match self.values.get(0) {
            Some(value) => value.clone(),
            None => return None,
        };

        match self.values.get(self.position) {
            Some(value) => {
                self.position += 1;
                Some((base, value.clone()))
            }

            None => {
                self.position = 0;
                self.values = &self.values[1..];
                self.next()
            }
        }
    }
}

fn main() {
    let values: Vec<u32> = (100..1000).collect();
    let pairs = Counter {
        values: &values,
        position: 0,
    };

    let result = pairs.filter_map(select_palindromes).max().unwrap();

    println!("{}", result);
}

fn select_palindromes((n1, n2): (u32, u32)) -> Option<u32> {
    let mult = n1 * n2;

    if mult == reverse(mult) {
        Some(mult)
    } else {
        None
    }
}

fn reverse(n: u32) -> u32 {
    let mut remaining = n;
    let mut reversed = 0;

    while remaining > 0 {
        reversed = reversed * 10 + (remaining % 10);
        remaining /= 10;
    }

    reversed
}
