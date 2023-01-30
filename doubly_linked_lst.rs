use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            prev: None,
            next: None,
        }
    }
}

impl Drop for ListNode {
    fn drop(&mut self) {
        println!("[+] Droping {}", self.val);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecordKeeper {
    record: HashMap<i32, Rc<RefCell<ListNode>>>,
}

impl RecordKeeper {
    fn new() -> Self {
        RecordKeeper {
            record: HashMap::new(),
        }
    }
    fn add_record(&mut self, val: i32, ptr: &Rc<RefCell<ListNode>>) {
        self.record
            .entry(val)
            .and_modify(|v| *v = Rc::clone(ptr))
            .or_insert(Rc::clone(ptr));
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

    fn insert_front(&mut self, val: i32, rk: &mut RecordKeeper) {
        self.size += 1;
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        rk.add_record(val, &new_node);
        if let Some(node) = self.head.take() {
            new_node.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        } else {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        }
    }

    fn insert_back(&mut self, val: i32, rk: &mut RecordKeeper) {
        self.size += 1;
        if let Some(node) = self.tail.take() {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            rk.add_record(val, &new_node);
            node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&node));
            self.tail = Some(new_node);
        } else {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            rk.add_record(val, &new_node);
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
    }
    #[allow(dead_code)]
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
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().val
        })
    }

    fn remove_node(&mut self, node: Rc<RefCell<ListNode>>) {
        println!("{}", Rc::strong_count(&node));
        self.size -= 1; // Since we have firm pointer to the node.. safe to decrease the size
                        // let prev_node_mut = Some(Rc::clone(node.borrow().prev.as_ref().unwrap()));
                        // let next_node_mut = Some(Rc::clone(node.borrow().next.as_ref().unwrap()));
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
                // Droping the tail
                self.tail.take();
                leftnode.borrow_mut().next = node.borrow_mut().next.take();
                node.borrow_mut().prev.take();
                self.tail = Some(Rc::clone(&leftnode));
            }
            (None, Some(rightnode)) => {
                // Droping the head
                self.head.take();
                node.borrow_mut().next.take();
                rightnode.borrow_mut().prev.take();
                self.head = Some(Rc::clone(&rightnode));
            }
            (None, None) => {
                // Only Node in the DLL
                self.head.take();
                self.tail.take();
                assert_eq!(self.size, 0);
            }
        }
        assert_eq!(Rc::strong_count(&node), 1);
    }

    fn traverse(&self) {
        if let Some(ref node) = self.head {
            //  Increase the strong_count, so that we operate on clone rather than old data
            let mut curr = Some(Rc::clone(node));
            while let Some(curr_node) = curr.take() {
                let _node = curr_node.borrow();
                println!("{}", _node.val);
                // Looks like tree setup..
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
    }
}

// Drop implementation to avoid memory leak
impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
    }
}

fn main() {
    let mut rk = RecordKeeper::new();
    let mut dll = DoublyLinkedList::new();
    dll.insert_back(1, &mut rk);
    dll.insert_back(2, &mut rk);
    dll.insert_back(3, &mut rk);
    dll.insert_back(4, &mut rk);

    dll.traverse();

    match rk.record.remove(&3) {
        Some(ptr) => {
            dll.remove_node(ptr);
        }
        None => {
            panic!("[!!] Key not found in the record..");
        }
    };
    println!("[*] Traversing after node drop..");
    dll.traverse();

    match rk.record.remove(&1) {
        Some(ptr) => {
            dll.remove_node(ptr);
        }
        None => {
            panic!("[!!] key not found in the record..");
        }
    };
    println!("[*] Traversing after second drop..");
    dll.traverse();

    dll.insert_front(0, &mut rk);

    println!("[+] Traversing after insertion at front");
    dll.traverse();

    let rec = rk.record.remove(&4).unwrap();
    dll.remove_node(rec);
    let rec = rk.record.remove(&0).unwrap();
    dll.remove_node(rec);
    let rec = rk.record.remove(&2).unwrap();
    dll.remove_node(rec);
    

}
