pub struct ArrayQueue<'a, T, const N: usize> {
    data: &'a mut [T; N],
    length: usize,
    write_pos: usize,
    read_pos: usize,
}

impl<'a, T, const N: usize> ArrayQueue<'a, T, N> where T: Default {
    pub fn new(data: &'a mut [T; N]) -> Self {
        Self {
            data,
            length: 0,
            write_pos: 0,
            read_pos: 0,
        }
    }
}

impl<'a, T, const N: usize> ArrayQueue<'a, T, N> {
    pub fn push(&mut self, value: T) -> Result<(), &str> {
        if self.length >= N {
            return Err("FULL");
        }

        self.data[self.write_pos] = value;
        self.length += 1;
        self.write_pos += 1;
        if self.write_pos == N {
            self.write_pos = 0
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Result<&T, &str> {
        if self.length == 0 {
            return Err("EMPTY");
        }

        let value = &self.data[self.read_pos];
        self.length -= 1;
        self.read_pos += 1;
        if self.read_pos == N {
            self.read_pos = 0
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue() {
        let data = &mut [-1; 3];
        let mut queue = ArrayQueue::<i32, 3>::new(data);

        assert_eq!(queue.pop(), Err("EMPTY"));

        assert_eq!(queue.push(1), Ok(()));
        assert_eq!(queue.push(2), Ok(()));
        assert_eq!(queue.pop(), Ok(&1));

        assert_eq!(queue.push(3), Ok(()));
        assert_eq!(queue.push(4), Ok(()));
        assert_eq!(queue.push(5), Err("FULL"));

        assert_eq!(queue.pop(), Ok(&2));
        assert_eq!(queue.pop(), Ok(&3));
        assert_eq!(queue.pop(), Ok(&4));
        assert_eq!(queue.pop(), Err("EMPTY"));
    }
}
