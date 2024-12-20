struct RLEIterator {
    encoding: Vec<i32>,
    index: usize,
    remaining: i32,
}

impl RLEIterator {
    pub fn new(encoding: Vec<i32>) -> Self {
        RLEIterator {
            encoding,
            index: 0,
            remaining: 0,
        }
    }

    pub fn next(&mut self, n: i32) -> i32 {
        let mut n = n;

        while self.index < self.encoding.len() {
            if self.remaining == 0 {
                self.remaining = self.encoding[self.index];
                self.index += 1;
            }

            if n <= self.remaining {
                self.remaining -= n;
                return self.encoding[self.index];
            } else {
                n -= self.remaining;
                self.remaining = 0;
                self.index += 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_iterator() {
        let mut iterator = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);

        assert_eq!(iterator.next(2), 8); // 2 elements of 8
        assert_eq!(iterator.next(1), 8); // 1 element of 8
        assert_eq!(iterator.next(1), 5); // 1 element of 5
        assert_eq!(iterator.next(2), -1); // No more elements
    }
}
