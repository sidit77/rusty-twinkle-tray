use std::fmt::{Debug, Formatter};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct OrderedMap<K, V> {
    items: Vec<(K, V)>
}

impl<K: PartialEq, V: SortKeyExtract> OrderedMap<K, V> {
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let prev = self.remove(&key);
        self.items.push((key, value));
        self.items
            .sort_by(|(_, a), (_, b)| a.sort_key().cmp(&b.sort_key()));
        prev
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(index) = self.items.iter().position(|(k, _)| k == key) {
            Some(self.items.remove(index).1)
        } else {
            None
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.items.iter().map(|(_, v)| v)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<K: Debug, V: Debug> Debug for OrderedMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.items.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

pub trait SortKeyExtract {
    type Key: Ord;
    fn sort_key(&self) -> Self::Key;
}
