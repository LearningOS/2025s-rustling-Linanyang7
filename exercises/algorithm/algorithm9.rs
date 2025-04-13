/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.items.len() <= self.count {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }
        self.sift_up(self.count);
    }

    fn parent_idx(idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        idx * 2 <= self.count
    }

    fn left_child_idx(idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn sift_up(&mut self, mut idx: usize) {
        let cmp = self.comparator;
        while idx > 1 && cmp(&self.items[idx], &self.items[Self::parent_idx(idx)]) {
            self.items.swap(idx, Self::parent_idx(idx));
            idx = Self::parent_idx(idx);
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let cmp = self.comparator;
        while self.children_present(idx) {
            let left = Self::left_child_idx(idx);
            let right = Self::right_child_idx(idx);
            let mut child = left;
            if right <= self.count && cmp(&self.items[right], &self.items[left]) {
                child = right;
            }
            if cmp(&self.items[child], &self.items[idx]) {
                self.items.swap(idx, child);
                idx = child;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        if self.count > 0 {
            self.sift_down(1);
        }
        self.items.truncate(self.count + 1);
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}    