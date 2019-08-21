
use std::collections::HashMap;
struct LRUCache {
    capacity: i32,
    first: *mut Node,
    last: *mut Node,
    nodeMap: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache{capacity, first: None, last: None, nodeMap: HashMap::new()}
    }

    fn get(&self, key: i32) -> i32 {
        -1
    }

    fn put(&self, key: i32, value: i32) {

    }
}

struct Node {
    pre: *mut Node,
    next: *mut Node,
    key: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let cache: LRUCache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));       // 返回  1
        cache.put(3, 3);    // 该操作会使得密钥 2 作废
        assert_eq!(-1, cache.get(2));         // 返回 -1 (未找到)
        cache.put(4, 4);    // 该操作会使得密钥 1 作废
        assert_eq!(-1, cache.get(1));       // 返回 -1 (未找到)
        assert_eq!(3, cache.get(3));       // 返回  3
        assert_eq!(4, cache.get(4));       // 返回  4
    }
}