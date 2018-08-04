pub struct Triplets {
    counter: Counter,
}

impl Iterator for Triplets {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<(usize, usize, usize)> {
        self.counter.find(is_triplet)
    }
}

fn is_triplet(&(a, b, c): &(usize, usize, usize)) -> bool {
    (c * c) == (a * a) + (b * b)
}

pub fn new() -> Triplets {
    let counter = Counter {
        last_c: 1,
        last_b: 0,
        last_a: 0,
    };
    Triplets { counter: counter }
}

struct Counter {
    last_c: usize,
    last_b: usize,
    last_a: usize,
}

impl Iterator for Counter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<(usize, usize, usize)> {
        if self.last_a > 0 {
            self.last_a -= 1;
        } else if self.last_b > 1 {
            let new_b = self.last_b - 1;

            self.last_b = new_b;
            self.last_a = new_b - 1;
        } else {
            let new_c = self.last_c + 1;

            self.last_c = new_c;
            self.last_b = new_c - 1;
            self.last_a = new_c - 2;
        }

        Some((self.last_a, self.last_b, self.last_c))
    }
}
