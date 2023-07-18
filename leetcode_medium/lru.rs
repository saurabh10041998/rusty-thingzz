use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct ListNode {
    val: (i32, i32),
    prev: Option<Weak<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: (i32, i32)) -> Self {
        ListNode {
            val,
            prev: None,
            next: None,
        }
    }
}

// Is all the ListNode gets deallocated at the end ??
impl Drop for ListNode {
    fn drop(&mut self) {
        println!("[*] Dropping the node");
    }
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(ListNode::new((-1, -1))));
        let tail = Rc::new(RefCell::new(ListNode::new((-1, -1))));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        DoublyLinkedList { head, tail }
    }
    fn insert_front(&mut self, new_node: Rc<RefCell<ListNode>>) {
        new_node.borrow_mut().next = self.head.borrow_mut().next.take();
        new_node.borrow_mut().prev = self.tail.borrow_mut().prev.take();
        self.tail.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        self.head.borrow_mut().next = Some(new_node);
    }
    fn delete_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let prev_node = node.borrow().prev.as_ref().map(|y| match y.upgrade() {
            Some(subnode) => subnode,
            None => unreachable!("[*] At least head dummy node should be there"),
        });
        let next_node = node.borrow().next.as_ref().map(|node| Rc::clone(node));

        match (prev_node, next_node) {
            (Some(leftnode), Some(rightnode)) => {
                leftnode.borrow_mut().next = node.borrow_mut().next.take();
                rightnode.borrow_mut().prev = node.borrow_mut().prev.take();
            }
            (_, _) => unreachable!(),
        }
    }
    fn pop_back(&mut self) -> (i32, i32) {
        let last_node = self
            .tail
            .borrow()
            .prev
            .as_ref()
            .map(|weak_node| match weak_node.upgrade() {
                Some(strong_node) => strong_node,
                None => unreachable!(),
            })
            .unwrap();
        let val = last_node.borrow().val;
        self.delete_node(last_node);
        val
    }
}

struct LRUCache {
    dll: DoublyLinkedList,
    record: HashMap<i32, Rc<RefCell<ListNode>>>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let record = HashMap::with_capacity(capacity as usize);
        let dll = DoublyLinkedList::new();
        LRUCache {
            dll,
            record,
            capacity: capacity as usize,
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        match self.record.remove(&key) {
            Some(record_ref) => {
                let val = record_ref.borrow().val;
                self.dll.delete_node(record_ref);
                let new_node = Rc::new(RefCell::new(ListNode::new(val)));
                self.dll.insert_front(Rc::clone(&new_node));
                self.record.entry(val.0).or_insert(new_node);
                println!("[*] After getting record: {:?}", self.record);
                val.1
            }
            None => -1,
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        println!(
            "Before: Key, val: {:?}, record: {:?}",
            (key, value),
            self.record.keys()
        );
        match self.record.get(&key) {
            Some(record_ref) => {
                record_ref.borrow_mut().val = (key, value);
            }
            None => {
                if self.record.len() == self.capacity {
                    // Delete least recently node
                    let (k, _) = self.dll.pop_back();
                    self.record.remove(&k).unwrap();
                }
                println!(
                    "After: Key, val: {:?}, record: {:?}",
                    (key, value),
                    self.record.keys()
                );
                let new_node = Rc::new(RefCell::new(ListNode::new((key, value))));
                self.dll.insert_front(Rc::clone(&new_node));
                self.record.entry(key).or_insert(new_node);
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        //assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), -1);
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    //assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), -1);
}
