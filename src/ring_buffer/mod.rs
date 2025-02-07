/*!
    This module contains a ring buffer implementation used by nodes in the network
*/

#![allow(unused)]

#[cfg(test)]
mod tests;

use std::collections::VecDeque;

/// ring buffer structure templated over the generic type T
#[derive(Debug)]
pub struct RingBuffer<T> {
    buff: VecDeque<T>,
    size: usize,
}

impl<T> RingBuffer<T> {
    /// constructor that allocate a buffer with specified capacity
    /// * size: requested capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buff: if size < 0x400 {
                VecDeque::with_capacity(size)
            } else {
                VecDeque::new()
            },
            size,
        }
    }

    /// inserts an element in the buffer as the last element
    ///
    /// removes the first element if buffer is full and returns it
    /// * e: element to insert in the buffer
    #[inline]
    pub fn insert(&mut self, e: T) -> Option<T> {
        let mut ret: Option<T> = None;
        if self.is_full() {
            ret = self.buff.pop_front();
        }

        self.buff.push_back(e);
        ret
    }

    /// removes the first element and returns it
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.buff.pop_front()
    }

    /// checks if the buffer is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buff.is_empty()
    }

    /// check if the buffer is full
    #[inline]
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.buff.len() == self.size
    }
}

impl<T: PartialEq> RingBuffer<T> {
    /// check if the buffer contains the passed element
    /// * e: element to be search inside the buffer
    pub fn contains(&self, e: &T) -> bool {
        self.buff.contains(e)
    }
}
