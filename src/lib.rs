mod store;

use std::hash::Hash;
pub use store::{LRUStrategy, Strategy};

pub struct Store<Key, Value> {
    strategy: Box<dyn Strategy<Key, Value>>,
}

impl<Key, Value> Store<Key, Value>
where
    Key: Eq,
    Key: Hash,
    Key: Copy,
{
    pub fn set(&mut self, key: Key, value: Value) -> bool {
        self.strategy.set(key, value)
    }
    pub fn get(&mut self, key: &Key) -> Option<&Value> {
        self.strategy.get(key)
    }
    pub fn delete(&mut self, key: &Key) -> Option<Value> {
        self.strategy.pop(key)
    }
    pub fn size(&self) -> usize {
        self.strategy.size()
    }
}

#[test]
fn simple_discard() {
    let mut store = Store {
        strategy: Box::new(LRUStrategy::with_capacity(2)),
    };
    store.set(2, 3);
    store.set(3, 3);
    store.set(4, 3);
    store.set(5, 3);
    assert_eq!(None, store.get(&2));
    assert_eq!(Some(&3), store.get(&3));
    assert_eq!(Some(&3), store.get(&4));
    assert_eq!(Some(&3), store.get(&5));
}

#[test]
fn simple() {
    let mut store = Store {
        strategy: Box::new(LRUStrategy::new()),
    };
    store.set(2, 3);
    store.set(3, 3);
    store.set(4, 3);
    store.set(5, 3);
    assert_eq!(Some(&3), store.get(&2));
    assert_eq!(Some(&3), store.get(&3));
    assert_eq!(Some(&3), store.get(&4));
    assert_eq!(Some(&3), store.get(&5));
}

#[test]
fn simple2() {
    let mut store = Store {
        strategy: Box::new(LRUStrategy::new()),
    };
    store.set(2, 3);
    store.set(3, 3);
    store.set(4, 3);
    store.set(5, 3);
    assert_eq!(Some(&3), store.get(&2));
    assert_eq!(Some(&3), store.get(&3));
    assert_eq!(Some(&3), store.get(&4));
    assert_eq!(Some(&3), store.get(&5));
}
