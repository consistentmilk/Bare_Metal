use std::collections::{hash_map::Entry, HashMap};

pub struct Node {
    /// The key of the node
    key: i32,

    /// The value of the node
    val: i32,

    /// Index of the previous node in the list
    prev: usize,

    /// Index of the next node in the list
    next: usize,
}

impl Node {
    pub fn new(key: i32, val: i32, prev: usize, next: usize) -> Self {
        Self {
            key,
            val,
            prev,
            next,
        }
    }
}

pub struct LRUCache {
    /// Index of the oldest item in the list
    lru: usize,

    /// Index of the newest item in the list
    mru: usize,

    /// list acts like a doubly linked list
    list: Vec<Node>,

    /// Mapping the `key` of `Node`s to their indices in the `list` vector
    map: HashMap<i32, usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            list: Vec::with_capacity(capacity as usize),
            lru: 0,
            mru: 0,
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            if self.mru == idx {
                return self.list[idx].val;
            }

            if self.lru != idx {
                let prev: usize = self.list[idx].prev;
                self.list[prev].next = self.list[idx].next;
            } else {
                self.lru = self.list[idx].next;
            }

            let next: usize = self.list[idx].next;
            self.list[next].prev = self.list[idx].prev;

            self.list[self.mru].next = idx;
            self.list[idx].prev = self.mru;
            self.mru = idx;

            self.list[idx].val
        } else {
            return -1;
        }
    }

    pub fn put(&mut self, key: i32, val: i32) {
        match self.map.entry(key) {
            Entry::Occupied(o) => {
                self.list[*o.get()].val = val;
                self.get(key);
            }

            Entry::Vacant(v) => {
                if self.list.capacity() > self.list.len() {
                    self.list.push(Node::new(key, val, self.mru, 0));

                    let idx: usize = self.list.len() - 1;

                    self.list[self.mru].next = idx;
                    self.mru = idx;

                    v.insert(idx);
                    return;
                }

                v.insert(self.lru);

                self.map.remove(&self.list[self.lru].key);

                self.list[self.lru].key = key;
                self.list[self.lru].val = val;
                self.get(key);
            }
        }
    }
}
