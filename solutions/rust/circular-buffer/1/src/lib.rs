use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    capacity: usize,
    elements: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            elements: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.elements.len() == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.elements.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.elements.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.elements.len() == self.capacity {
            self.elements.pop_front();
        }
        if self.capacity > 0 {
            self.elements.push_back(element);
        }
    }
}
