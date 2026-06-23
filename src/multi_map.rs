use std::collections::{
    hash_map::{Entry, RandomState},
    HashMap,
};
use std::hash::Hash;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
struct Node<V> {
    value: V,
    next: Option<NonZeroUsize>,
}

impl<V> Node<V> {
    pub fn new(value: V) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug, Clone)]
struct Pointers {
    head: usize,
    tail: usize,
}

#[derive(Debug, Clone)]
pub struct LinkedMultiHashMap<K, V, S = RandomState> {
    map: HashMap<K, Pointers, S>,
    nodes: Vec<Node<V>>,
}

impl<K, V> Default for LinkedMultiHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> LinkedMultiHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }
}

pub struct GetIter<'s, V> {
    current: Option<usize>,
    nodes: &'s Vec<Node<V>>,
}

impl<'s, V> Iterator for GetIter<'s, V> {
    type Item = &'s V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.current.take() {
            let node = &self.nodes[i];

            if let Some(next_i) = node.next {
                self.current = Some(next_i.get() - 1);
            }

            Some(&node.value)
        } else {
            None
        }
    }
}

impl<K: Eq + Hash, V> LinkedMultiHashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        let len = self.nodes.len();

        match self.map.entry(key) {
            Entry::Occupied(mut entry) => {
                let tail = &mut entry.get_mut().tail;
                let new_node = Node::new(value);
                self.nodes[*tail].next = Some(NonZeroUsize::new(len + 1).unwrap());
                *tail = len;
                self.nodes.push(new_node);
            }
            Entry::Vacant(entry) => {
                entry.insert(Pointers {
                    head: len,
                    tail: len,
                });
                self.nodes.push(Node::new(value));
            }
        };
    }

    #[inline]
    pub fn get_iter(&self, key: &K) -> Option<GetIter<'_, V>> {
        self.map.get(key).map(|pointers| GetIter {
            current: Some(pointers.head),
            nodes: &self.nodes,
        })
    }
}

impl<K, V, S> LinkedMultiHashMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            map: HashMap::with_hasher(hash_builder),
            nodes: Vec::new(),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            nodes: Vec::with_capacity(capacity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_multi_hash_map() {
        let mut map = LinkedMultiHashMap::new();

        map.insert("a", 1);
        map.insert("b", 1);
        map.insert("a", 2);
        map.insert("a", 3);
        map.insert("b", 2);
        map.insert("c", 1);

        dbg!(&map);

        assert!(map.get_iter(&"d").is_none());
        assert_eq!(
            map.get_iter(&"a").unwrap().collect::<Vec<_>>(),
            vec![&1, &2, &3]
        );
        assert_eq!(
            map.get_iter(&"b").unwrap().collect::<Vec<_>>(),
            vec![&1, &2]
        );
        assert_eq!(map.get_iter(&"c").unwrap().collect::<Vec<_>>(), vec![&1]);
    }
}
