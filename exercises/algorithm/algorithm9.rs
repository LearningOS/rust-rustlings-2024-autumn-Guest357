use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
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
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count - 1);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if left < self.count {
            if right < self.count {
                if (self.comparator)(&self.items[left], &self.items[right]) {
                    Some(left)
                } else {
                    Some(right)
                }
            } else {
                Some(left)
            }
        } else {
            None
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut parent_idx = idx;
        while let Some(child_idx) = self.smallest_child_idx(parent_idx) {
            if (self.comparator)(&self.items[parent_idx], &self.items[child_idx]) {
                break;
            }
            self.items.swap(parent_idx, child_idx);
            parent_idx = child_idx;
        }
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut child_idx = idx;
        while child_idx > 0 {
            let parent_idx = self.parent_idx(child_idx);
            if (self.comparator)(&self.items[child_idx], &self.items[parent_idx]) {
                self.items.swap(child_idx, parent_idx);
                child_idx = parent_idx;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let top = self.items.swap_remove(0);
            self.count -= 1;
            self.bubble_down(0);
            Some(top)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
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
    }
}