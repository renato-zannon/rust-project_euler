pub struct Combinations<A> {
    values: Vec<A>,
    last_pair: Option<(uint, uint)>
}

pub fn new<A>(values: Vec<A>) -> Combinations<A> {
    Combinations {
        values: values,
        last_pair: None
    }
}

impl<A: Clone> Iterator<(A, A)> for Combinations<A> {
    fn next(&mut self) -> Option<(A, A)> {
        let perm = self.next_pair();
        self.last_pair = perm;

        perm.map(|(next_left, next_right)| {
            let ref left_value  = self.values[next_left];
            let ref right_value = self.values[next_right];

            (left_value.clone(), right_value.clone())
        })
    }
}

impl<A> Combinations<A> {
    fn next_pair(&self) -> Option<(uint, uint)> {
        return match self.last_pair {
            None => Some((0, 0)),

            Some((last_left, last_right)) => {
                let max = self.values.len() - 1;

                if last_right < max {
                    Some((last_left, last_right + 1))
                } else if last_left < max {
                    Some((last_left + 1, 0))
                } else {
                    None
                }
            }
        };
    }
}
