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

    pub fn push(&mut self, value: T) {
        self.vec.push((value, 0));
        self.recursive_fold();
    }

    pub fn count(&self) -> usize {
        self.vec
            .iter()
            .map(|(_, e)| (2usize).pow(*e as u32))
            .sum::<usize>()
    }

    fn recursive_fold(&mut self) {
        if self.vec.len() == 1 {
            return;
        }

        // check the last two elements if they have the same counter
        let last_1 = self.vec.pop().unwrap();
        let last_2 = self.vec.pop().unwrap();

        if last_1.1 == last_2.1 {
            let result = (self.f)(last_1.0, last_2.0);
            let counter = last_1.1 + 1;

            self.vec.push((result, counter));
            self.recursive_fold();
        }

        else {
            // push the values back
            self.vec.push(last_2);
            self.vec.push(last_1);
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
