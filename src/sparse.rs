/// Implementation of data structures revolving around the concept of a
/// "Sparse Set", as famously described here:
/// https://research.swtch.com/sparse
pub struct SparseMap<T> {
    sparse: Vec<usize>,
    dense: Vec<usize>,
    values: Vec<T>,
}

impl<T> SparseMap<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: vec![0; capacity],
            dense: vec![0; capacity],
            values: Vec::with_capacity(capacity),
        }
    }

    /// Only `O(1)` if `T` is not [`Drop`].
    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn insert(&mut self, member: usize, value: T) {
        let index = self.sparse[member];
        let size = self.values.len();

        if index < size && self.dense[index] == member {
            self.values[index] = value;
        } else {
            self.dense[size] = member;
            self.sparse[member] = size;
            self.values.push(value);
        }
    }
}
