use std::{collections::HashMap, hash::Hash};

pub trait Strategy<Key, Value> {
    fn get(&mut self, key: &Key) -> Option<&Value>;
    fn set(&mut self, key: Key, value: Value) -> bool;
    fn size(&self) -> usize;
    fn pop(&mut self, key: &Key) -> Option<Value>;
}

pub struct LRUStrategy<Key, Value> {
    cache: HashMap<Key, Value>,
    order: Vec<Key>,
    capacity: usize,
}

impl<Key, Value> LRUStrategy<Key, Value>
where
    Key: Eq,
    Key: Hash,
    Key: Copy,
{
    fn increment_key(&mut self, key: &Key) {
        self.order.retain(|key_in_order| key_in_order != key);
        self.order.insert(0, *key);
    }

    pub fn with_capacity(size: usize) -> LRUStrategy<Key, Value> {
        LRUStrategy {
            cache: HashMap::new(),
            order: vec![],
            capacity: size,
        }
    }

    pub fn new() -> Self{
        Default::default()
    }

}

impl<Key, Value> Default for LRUStrategy<Key, Value>
where
    Key: Eq,
    Key: Hash,
    Key: Copy,
{
    fn default() -> Self {
        LRUStrategy {
            cache: HashMap::new(),
            order: vec![],
            capacity: 0,
        }
    }
}

impl<Key, Value> Strategy<Key, Value> for LRUStrategy<Key, Value>
where
    Key: Eq,
    Key: Hash,
    Key: Copy,
{
    fn get(&mut self, key: &Key) -> Option<&Value> {
        match self.cache.contains_key(key) {
            true => {
                self.increment_key(key);
                Some(&self.cache[key])
            }
            false => None,
        }
    }

    fn set(&mut self, key: Key, value: Value) -> bool {
        match self.cache.contains_key(&key) {
            true => {
                self.increment_key(&key);
                self.cache.insert(key, value);
                true
            }
            false => {
                if self.capacity != 0 && self.order.len() > self.capacity {
                    let last_accessed = self.order.pop().unwrap();
                    self.cache.remove(&last_accessed);
                }
                self.cache.insert(key, value);
                self.increment_key(&key);
                true
            }
        }
    }

    fn size(&self) -> usize {
        self.order.len()
    }

    fn pop(&mut self, key: &Key) -> Option<Value> {
        let result = self.cache.remove(&key);
        if result.is_some() {
            self.order.retain(|key_in_order| key_in_order != key);
            result
        } else {
            None
        }
    }
}
