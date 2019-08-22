
use std::collections::HashMap;
use std::ptr;
use std::mem;

// Entry is either a map entry and a link-list node
pub struct LRUEntry {
    key: i32,
    val: i32,
    prev: *mut LRUEntry,
    next: *mut LRUEntry,
}

impl LRUEntry {
    pub fn new(key: i32, val: i32) -> Self {
        LRUEntry{
            key: key,
            val: val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

pub struct LRUCache {
    map: HashMap<i32, Box<LRUEntry>>,
    cap: usize,

    // head and tail is dummy node of the double-linked-list
    head: *mut LRUEntry,
    tail: *mut LRUEntry,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let map = HashMap::with_capacity(capacity);
        let cache = LRUCache{
            map: map,
            cap: capacity,
            head: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
            tail: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
        };
        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let (ptr, val) = match self.map.get_mut(&key) {
            None => (None, None),
            Some(entry) => {
                let ptr: *mut LRUEntry = &mut **entry;
                (Some(ptr), Some(unsafe { (*entry).val }))
            }
        };

        if let Some(ptr) = ptr {
            self.detach(ptr);
            self.push(ptr);
        }
        val.unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let ptr = self.map.get_mut(&key).map(|entry| {
            let ptr: *mut LRUEntry = &mut **entry;
            ptr
        });

        match ptr {
            Some(ptr) => {
                // key already exist, update it
                unsafe { (*ptr).val = value };
                self.detach(ptr);
                self.push(ptr);
            }
            None => {
                // insert new key, cache is full, evict
                if self.map.len() == self.cap {
                    let mut old_entry = self.pop().unwrap();
                    old_entry.key = key;
                    old_entry.val = value;
                    self.push(&mut *old_entry);
                    self.map.insert(key, old_entry);
                } else {
                    let mut new_entry = Box::new(LRUEntry::new(key, value));
                    self.push(&mut *new_entry);
                    self.map.insert(key, new_entry);
                }
            }
        }
    }

    // pop() remove the entry from map, detach the entry from head of linked-list, and return it
    fn pop(&mut self) -> Option<Box<LRUEntry>> {
        let next;
        unsafe { next = (*self.head).next }
        // list is empty
        if next == self.tail {
            return None
        }
        let key = unsafe { (*next).key };
        let mut old_entry = self.map.remove(&key).unwrap();
        self.detach(&mut *old_entry);
        Some(old_entry)
    }

    // push() pushs an entry to the tail of linked-list
    fn push(&mut self, entry: *mut LRUEntry) {
        unsafe {
            // prev <-> tail
            // prev <-> entry <-> tail
            (*entry).prev = (*self.tail).prev;
            (*entry).next = self.tail;
            (*self.tail).prev = entry;
            (*(*entry).prev).next = entry;
        }
    }

    // detach() remove an entry from the linked-list
    fn detach(&mut self, entry: *mut LRUEntry) {
        unsafe {
            // prev <-> entry <-> next
            // prev <-> next
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
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