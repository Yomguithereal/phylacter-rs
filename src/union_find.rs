/// Implementation of a Disjoint Set or Union Find data structure to keep track
/// of connected components in a graph.
///
/// I use union by size here, not by rank, because it means you can very easily
/// track connected component sizes in number of nodes, which is usually more
/// useful than tracking node "ranks".
///
/// Ref: https://en.wikipedia.org/wiki/Disjoint-set_data_structure

#[derive(Debug, Clone)]
pub struct UnionFindEntry {
    parent: usize,
    size: usize,
}

impl UnionFindEntry {
    #[inline(always)]
    pub fn parent(&self) -> usize {
        self.parent
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Default)]
pub struct UnionFind {
    entries: Vec<UnionFindEntry>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut entries = Vec::new();

        for i in 0..capacity {
            entries.push(UnionFindEntry { parent: i, size: 1 });
        }

        Self { entries }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    pub fn make_set(&mut self) -> usize {
        let i = self.entries.len();

        self.entries.push(UnionFindEntry { parent: i, size: 1 });

        i
    }

    #[inline]
    pub fn find(&self, x: usize) -> usize {
        let mut root = x;

        loop {
            let parent = self.entries[root].parent;

            if parent == root {
                break;
            }

            root = parent;
        }

        root
    }

    fn find_mut(&mut self, mut x: usize) -> usize {
        let mut root = x;

        loop {
            let parent = self.entries[root].parent;

            if parent == root {
                break;
            }

            root = parent;
        }

        // Path compression
        loop {
            let entry = &mut self.entries[x];

            if entry.parent == root {
                break;
            }

            let parent = entry.parent;
            entry.parent = root;
            x = parent;
        }

        root
    }

    #[inline]
    pub fn union(&mut self, mut x: usize, mut y: usize) {
        x = self.find_mut(x);
        y = self.find_mut(y);

        if x == y {
            return;
        }

        let x_size = self.entries[x].size;
        let y_size = self.entries[y].size;

        if x_size > y_size {
            self.entries[y].parent = x;
            self.entries[x].size += y_size;
        } else {
            self.entries[x].parent = y;
            self.entries[y].size += x_size;
        }
    }

    /// Iterate over "leaders", i.e. root nodes of each connected component in the
    /// graph. It is quite arbitrary and depends on insertion order.
    ///
    /// Node id can be found using [`UnionFindEntry::parent`].
    #[inline]
    pub fn leaders(&self) -> impl Iterator<Item = &UnionFindEntry> {
        self.entries.iter().enumerate().filter_map(|(i, entry)| {
            if i != entry.parent {
                None
            } else {
                Some(entry)
            }
        })
    }

    /// Find the entry representing the root node of the largest connected
    /// component in the graph.
    #[inline]
    pub fn largest_entry(&self) -> Option<&UnionFindEntry> {
        self.leaders().max_by(|a, b| a.size.cmp(&b.size))
    }

    /// Iterate over connected component sizes.
    #[inline]
    pub fn sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.leaders().map(|entry| entry.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut union_find = UnionFind::new();

        assert_eq!(union_find.is_empty(), true);

        union_find.make_set();
        union_find.make_set();
        union_find.make_set();

        assert_eq!(union_find.len(), 3);

        assert_eq!(union_find.sizes().collect::<Vec<_>>(), vec![1, 1, 1]);

        union_find.union(0, 1);
        union_find.union(0, 2);

        assert_eq!(union_find.largest_entry().unwrap().size, 3);
        assert_eq!(union_find.sizes().collect::<Vec<_>>(), vec![3]);
    }
}
