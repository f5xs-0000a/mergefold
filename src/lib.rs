/// A structure that folds over `T` in a way similar to the game 2048.
///
/// Each `T` along with an exponent of 0 are pushed into the stack.
/// If there are two of `T` next to each other with the same exponent, the two
/// `T` are folded using `F` and the exponent is incremented.
///
/// Note that `F` does not have context of the magnitude of each `T`, so folding
/// two of `T` each with the same exponent is the same as folding two of `T`
/// with really big exponent differences.
pub struct MergeFold<T, F>
where F: Fn(T, T) -> T {
    vec: Vec<(T, u8)>,
    f: F,
}

impl<T, F> MergeFold<T, F>
where F: Fn(T, T) -> T {
    /// Creates a new MergeFold object generic over object `T` and a folding
    /// method `F`.
    pub fn new(fold: F) -> MergeFold<T, F> {
        MergeFold {
            vec: vec![],
            f: fold,
        }
    }

    /// Counts the number of pushes done.
    ///
    /// In implementation, this just sums the power of 2 of exponents.
    pub fn count(&self) -> usize {
        self.vec
            .iter()
            .map(|(_, e)| (2usize).pow(*e as u32))
            .sum::<usize>()
    }

    /// Pushes an object `T` into the stack.
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

    /// Folds the entire stack. 
    ///
    /// Note that `F` does not have context of the magnitude of each `T`, so
    /// folding two of `T` each with the same exponent is the same as folding
    /// two of `T` with really big exponent differences.
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
