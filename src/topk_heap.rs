use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::cmp::Arbitrary;

#[derive(Clone, Debug)]
pub struct TopKHeapMap<T, V> {
    k: usize,
    heap: BinaryHeap<(Reverse<T>, Arbitrary<V>)>,
}

impl<T: Ord, V> TopKHeapMap<T, V> {
    #[inline]
    pub fn new(k: usize) -> Self {
        Self {
            k,
            heap: BinaryHeap::with_capacity(k),
        }
    }

    #[inline(always)]
    pub fn k(&self) -> usize {
        self.k
    }

    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn push_with<F>(&mut self, item: T, callback: F) -> bool
    where
        F: FnOnce() -> V,
    {
        let heap = &mut self.heap;

        if heap.len() < self.k {
            heap.push((Reverse(item), Arbitrary(callback())));

            return true;
        } else {
            let worst_item = heap.peek().unwrap();

            if item > worst_item.0 .0 {
                heap.pop();
                heap.push((Reverse(item), Arbitrary(callback())));
                return true;
            }
        }

        false
    }

    pub fn into_sorted_vec(mut self) -> Vec<(T, V)> {
        let l = self.heap.len();

        let mut items = Vec::with_capacity(l);
        let uninit = items.spare_capacity_mut();

        let mut i: usize = l;

        while let Some((Reverse(item), Arbitrary(value))) = self.heap.pop() {
            i -= 1;
            uninit[i].write((item, value));
        }

        unsafe {
            items.set_len(l);
        }

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut heap = TopKHeapMap::new(2);
        heap.push_with(1, || "one");
        heap.push_with(2, || "two");
        heap.push_with(3, || "three");

        assert_eq!(
            heap.clone().into_sorted_vec(),
            vec![(3, "three"), (2, "two")]
        );
    }
}
