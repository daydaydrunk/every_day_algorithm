use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

const INITIAL_CAPACITY: usize = 16;

pub struct HashTable<K, V> {
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    pub fn new() -> Self {
        HashTable {
            buckets: vec![LinkedList::new(); INITIAL_CAPACITY],
            size: 0,
        }
    }

    fn hash<Q: ?Sized>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        for &mut (ref existing_key, ref mut existing_value) in &mut self.buckets[index] {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }
        self.buckets[index].push_back((key, value));
        self.size += 1;
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        for &(ref existing_key, ref existing_value) in &self.buckets[index] {
            if existing_key.borrow() == key {
                return Some(existing_value);
            }
        }
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];
        let mut cursor = bucket.cursor_front_mut();
        while let Some((existing_key, _)) = cursor.current() {
            if existing_key.borrow() == key {
                self.size -= 1;
                return cursor.remove_current().map(|(_, v)| v);
            }
            cursor.move_next();
        }
        None
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
