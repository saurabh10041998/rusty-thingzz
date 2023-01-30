use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    key: i32,
    val: i32,
    cnt: usize,
    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        ListNode {
            key,
            val,
            cnt: 1,
            next: None,
            prev: None,
        }
    }
}

// Just for Debugg, Am I dropping all nodes created ??
impl Drop for ListNode {
    fn drop(&mut self) {
        println!("[+] Dropping Key-value: {} -> {}", self.key, self.val);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DoublyLinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    size: usize,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    fn insert_front(&mut self, node: Rc<RefCell<ListNode>>) {
        self.size += 1;
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(Rc::clone(&prev_head));
            self.head = Some(node);
        } else {
            self.tail = Some(Rc::clone(&node));
            self.head = Some(node);
        }
    }
    fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().key
        })
    }

    fn remove_node(&mut self, node: Rc<RefCell<ListNode>>) {
        self.size -= 1;
        let prev_node = match node.borrow().prev.as_ref() {
            Some(subnode) => Some(Rc::clone(subnode)),
            None => None,
        };
        let next_node = match node.borrow().next.as_ref() {
            Some(subnode) => Some(Rc::clone(subnode)),
            None => None,
        };
        match (prev_node, next_node) {
            (Some(leftnode), Some(rightnode)) => {
                leftnode.borrow_mut().next = node.borrow_mut().next.take();
                rightnode.borrow_mut().prev = node.borrow_mut().prev.take();
            }
            (Some(leftnode), None) => {
                // Droping the tail node
                self.tail.take();
                leftnode.borrow_mut().next = node.borrow_mut().next.take();
                node.borrow_mut().prev.take();
                self.tail = Some(leftnode);
            }
            (None, Some(rightnode)) => {
                // Droping the head node
                self.head.take();
                rightnode.borrow_mut().prev.take();
                node.borrow_mut().next.take();
                self.head = Some(rightnode);
            }
            (None, None) => {
                // Only node in DLL
                self.head.take();
                self.tail.take();
                assert_eq!(self.size, 0);
            }
        }
    }

    #[allow(dead_code)]
    fn traverse(&self) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(ref node) = self.head {
            let mut curr = Some(Rc::clone(node));
            while let Some(curr_node) = curr.take() {
                let _node = curr_node.borrow();
                ans.push(_node.key);
                match _node.next {
                    Some(ref subnode) => {
                        curr = Some(Rc::clone(subnode));
                    }
                    None => {
                        curr = None;
                    }
                }
            }
        }
        ans
    }
}

// MORE IMPORTANT: Drop to avoid memory leak
impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LFUCache {
    key_node: HashMap<i32, Rc<RefCell<ListNode>>>,
    freq_list_map: HashMap<usize, Box<DoublyLinkedList>>,
    max_cap: usize,
    min_freq: usize,
    curr_size: usize,
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        LFUCache {
            key_node: HashMap::new(),
            freq_list_map: HashMap::new(),
            max_cap: capacity,
            min_freq: 0,
            curr_size: 0,
        }
    }

    fn promote_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let key = node.borrow_mut().key;
        let cnt = node.borrow_mut().cnt;
        let dll = self.freq_list_map.get_mut(&cnt).unwrap();
        dll.remove_node(Rc::clone(&node));
        if cnt == self.min_freq && dll.size == 0 {
            self.min_freq += 1;
        }
        let mut next_higher_freq_lst = match self.freq_list_map.remove(&(cnt + 1)) {
            Some(lst) => lst,
            None => Box::new(DoublyLinkedList::new()),
        };
        node.borrow_mut().cnt += 1;
        next_higher_freq_lst.insert_front(Rc::clone(&node));
        self.freq_list_map
            .entry(cnt + 1)
            .or_insert(next_higher_freq_lst);
        self.key_node.entry(key).or_insert(Rc::clone(&node));
    }

    fn get(&mut self, key: i32) -> i32 {
        let is_present = self.key_node.contains_key(&key);
        if !is_present {
            return -1;
        }
        let node = Rc::clone(self.key_node.get(&key).unwrap());
        let val = node.borrow().val;
        self.promote_node(node);
        val
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.max_cap == 0 {
            return;
        }
        if self.key_node.contains_key(&key) {
            let node = Rc::clone(self.key_node.get(&key).unwrap());
            node.borrow_mut().val = value;
            self.promote_node(node);
        } else {
            if self.curr_size == self.max_cap {
                let dll = self.freq_list_map.get_mut(&self.min_freq).unwrap();
                let key = dll.tail.as_ref().unwrap().borrow().key;
                self.key_node.remove(&key);
                dll.pop_back();
                self.curr_size -= 1;
            }
            self.curr_size += 1;
            // Completly new node available
            self.min_freq = 1;
            let mut list_freq = match self.freq_list_map.remove(&self.min_freq) {
                Some(list) => list,
                None => Box::new(DoublyLinkedList::new()),
            };
            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            list_freq.insert_front(Rc::clone(&new_node));
            self.freq_list_map.entry(self.min_freq).or_insert(list_freq);
            self.key_node.entry(key).or_insert(new_node);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_dll() {
        // Inplace unit test to check DLL implementation
        let mut dll = DoublyLinkedList::new();
        let x = Rc::new(RefCell::new(ListNode::new(4, 1)));
        dll.insert_front(Rc::clone(&x));
        dll.insert_front(Rc::new(RefCell::new(ListNode::new(3, 1))));
        dll.insert_front(Rc::new(RefCell::new(ListNode::new(2, 1))));
        dll.insert_front(Rc::new(RefCell::new(ListNode::new(1, 1))));
        dll.insert_front(Rc::new(RefCell::new(ListNode::new(0, 1))));

        assert_eq!(dll.traverse(), vec![0, 1, 2, 3, 4]);

        // dll.pop_back();
        // assert_eq!(dll.traverse(), vec![0, 1, 2, 3]);

        // TODO: harden the behaviour of remove_node
        dll.remove_node(x);
        assert_eq!(dll.traverse(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn run_tc1() {
        let capacity = 2;
        let mut lfucache = LFUCache::new(capacity);
        lfucache.put(1, 1);
        lfucache.put(2, 2);
        assert_eq!(lfucache.get(1), 1);
        lfucache.put(3, 3);
        assert_eq!(lfucache.get(2), -1);
        assert_eq!(lfucache.get(3), 3);
        lfucache.put(4, 4);
        assert_eq!(lfucache.get(1), -1);
        assert_eq!(lfucache.get(3), 3);
        assert_eq!(lfucache.get(4), 4);
    }
}

fn main() {
    let capacity = 2;
    let mut lfucache = LFUCache::new(capacity);
    lfucache.put(1, 1);
    lfucache.put(2, 2);
    assert_eq!(lfucache.get(1), 1);
    lfucache.put(3, 3);
    assert_eq!(lfucache.get(2), -1);
    assert_eq!(lfucache.get(3), 3);
    lfucache.put(4, 4);
    assert_eq!(lfucache.get(1), -1);
    assert_eq!(lfucache.get(3), 3);
    assert_eq!(lfucache.get(4), 4);
}
