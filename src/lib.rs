pub struct MergeFold<T, F>
where F: Fn(T, T) -> T {
    vec: Vec<(T, u8)>,
    f: F,
}

impl<T, F> MergeFold<T, F>
where F: Fn(T, T) -> T {
    pub fn new(fold: F) -> MergeFold<T, F> {
        MergeFold {
            vec: vec![],
            f: fold,
        }
    }

    pub fn count(&self) -> usize {
        self.vec
            .iter()
            .map(|(_, e)| (2usize).pow(*e as u32))
            .sum::<usize>()
    }

    pub fn push(&mut self, value: T) {
        self.push_recursive(value, 0);
    }

    fn push_recursive(&mut self, value: T, counter: u8) {
        let mut should_pop = false;
        
        // check the last element if its counter is the same as the queried
        // counter. if it's the same, we can fold them together. if it's not,
        // we append to the vector. if the last doesn't exist, we still append
        // to the vector.
        if let Some(last) = self.vec.last() {
            if last.1 == counter {
                should_pop = true;
            }
        }

        // goes here if the counter of the last element is the same as the
        // queried counter
        if should_pop {
            let (last_value, _) = self.vec.pop().unwrap();
            let value = (self.f)(last_value, value);
            let counter = counter + 1;

            self.push_recursive(value, counter);
        }

        // goes here if the counter of the last element is not the same as the
        // queried counter OR the last element doesn't exist yet
        else {
            self.vec.push((value, counter));
        }
    }

    pub fn fold(self) -> Option<T> {
        // we're reversing so we get to emphasize the lower values first
        let (f, vec) = (self.f, self.vec);
        let mut iter = vec.into_iter().map(|(v, _)| v).rev();
        let first_value = match iter.next() {
            Some(x) => x,
            None => return None,
        };

        Some(
            iter.fold(first_value, |x, acc| (f)(x, acc))
        )
    }
}
