
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct LRUCache {
    capacity: usize,
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    map: HashMap<i32, Box<Node>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let cache = LRUCache {
            capacity: capacity as usize,
            head: None,
            tail: None,
            map: HashMap::with_capacity(capacity as usize)
        };
        cache
    }

    fn get(&mut self, key: i32) -> i32 {
         let v = match self.map.get_mut(&key) {
            Some(node) => {
                self.detach(node);
                self.push(node);
                node.value
            }
            None => {
                -1
            }
        };
        v
    }

    fn put(&mut self, key: i32, value: i32) {
        let node = self.map.get_mut(&key);

        match node {
            //不存在则添加
            None => {
                //如果满了，需要pop一个
                if self.map.len() == self.capacity {
                    self.pop();
                }
                let mut new_node = Box::new(Node::new(key, value));
                self.push(new_node);
                self.map.insert(key, new_node );
            }
            Some(n) =>{
                self.detach(n);
                self.push(n);
            }

        }
    }
    // pop() remove the head entry from map, detach the entry from head of linked-list, and return it
    fn pop(&mut self) -> Option<Box<Node>> {
        let mut first = self.head;
//        if first == None {
//            return None;
//        }

        self.map.remove(&first.unwrap().key);
        self.detach(first.unwrap());
        first

    }
    // detach() remove an entry from the linked-list
    fn detach(&mut self, mut node: Box<Node>) {
        node.pre.unwrap().next = node.next;
        node.next.unwrap().pre = node.pre;
    }
    // push() pushs an entry to the tail of linked-list
    fn push(&mut self, mut node:  Box<Node>) {
        if self.head == None {
            self.head = Some(node);
        }

    }

}
#[derive(Clone, Debug)]
struct Node {
    pre: Option<Box<Node>>,
    next: Option<Box<Node>>,
    key: i32,
    value: i32,
}
impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node{
            key,
            value: val,
            pre: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cache: LRUCache = LRUCache::new(2);
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