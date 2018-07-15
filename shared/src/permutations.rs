use std::marker::PhantomData;

pub trait Permutations<A>
where
    Self: Sized + AsMut<[A]> + AsRef<[A]>,
{
    fn permutations(self) -> SEPA<Self, A>;
}

impl<A, T> Permutations<A> for T
where
    T: AsMut<[A]> + AsRef<[A]>,
    A: Ord + Clone,
{
    fn permutations(self) -> SEPA<T, A> {
        SEPA {
            current: self,
            first: true,
            _marker: PhantomData,
        }
    }
}

// http://permute.tchs.info/soda_submit.php
pub struct SEPA<T, A>
where
    T: AsMut<[A]> + AsRef<[A]>,
{
    current: T,
    first: bool,

    _marker: PhantomData<[A]>,
}

impl<A: Ord + Clone, T: AsMut<[A]> + AsRef<[A]>> Iterator for SEPA<T, A> {
    type Item = Vec<A>;

    fn next(&mut self) -> Option<Vec<A>> {
        self.permute().map(|slice| slice.to_vec())
    }
}

impl<A: Ord + Clone, T: AsMut<[A]> + AsRef<[A]>> SEPA<T, A> {
    pub fn permute<'a>(&'a mut self) -> Option<&'a mut [A]> {
        if self.first {
            self.first = false;
            return Some(self.current.as_mut());
        }

        if let Some((key, newkey)) = self.next_keys() {
            let slice = self.current.as_mut();

            permute(slice, key, newkey);
            return Some(slice);
        }

        None
    }

    fn next_keys(&self) -> Option<(usize, usize)> {
        let current_perm = self.current.as_ref();
        let current_len = current_perm.len();

        let maybe_key_index: Option<usize> = (1..current_len)
            .rev()
            .find(|&index| {
                let ref element = current_perm[index];
                let ref element_before = current_perm[index - 1];

                element > element_before
            })
            .map(|after_key_index| after_key_index - 1);

        maybe_key_index.and_then(|key_index| {
            let ref key_element = current_perm[key_index];

            (key_index + 1..current_len)
                .filter(|&index| {
                    let ref element = current_perm[index];
                    element > key_element
                })
                .min_by(|&index1, &index2| current_perm[index1].cmp(&current_perm[index2]))
                .map(|newkey| (key_index, newkey))
        })
    }
}

fn permute<T>(slice: &mut [T], key: usize, newkey: usize) {
    slice.swap(key, newkey);

    let mut from_start = key + 1;
    let mut from_end = slice.len() - 1;

    while from_end > from_start {
        slice.swap(from_end, from_start);
        from_end -= 1;
        from_start += 1;
    }
}
