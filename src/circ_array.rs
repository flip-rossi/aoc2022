use std::{mem, cmp::min};

pub struct CircArray<T> {
    buf: Vec<Option<T>>,
    capacity: usize, size: usize,
    head: usize, tail: usize,
}

impl<T> CircArray<T> {
    pub fn new(capacity: usize) -> Self {
        CircArray {
            buf: {
                let mut buf = Vec::with_capacity(capacity);
                for _ in 0..capacity {
                    buf.push(None);
                }
                buf
            },
            capacity, size: 0,
            head: 0, tail: 0,
        }
    }

    pub fn add_last(&mut self, val: T) -> Option<T> {
        self.tail = self.next_index(self.tail);
        if self.tail == self.head {
            self.head = self.next_index(self.head);
        }
        self.size = min(self.size + 1, self.capacity);

        let dropped = mem::replace(
            &mut self.buf[self.tail],
            Some(val)
        );

        dropped
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let value = mem::replace(&mut self.buf[self.head], None);
        if let Some(_) = value {
            self.head = self.next_index(self.head);
            self.size -= 1;
        }

        value
    }

    // Private methods
    fn next_index(&self, index: usize) -> usize {
        (index+1) % self.capacity
    }
}

impl<T> CircArray<T> where T: Eq {
    pub fn has_duplicates(&self) -> bool {
        for i in 0..self.capacity-1 {
            for j in i+1..self.capacity {
                if self.buf[i] == self.buf[j] { return true }
            }
        }
        false
    }
}

