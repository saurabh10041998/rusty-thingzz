use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Debug, PartialEq, Eq)]
pub struct DoublyLinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList { 
            head: None, 
            tail: None 
        }
    }
    fn insert_back(&mut self, val: i32) {
        if let Some(node) = self.tail.take() {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&node));
            self.tail = Some(new_node);
        } else {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);            
        }
    }
    fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|prev_tail|  {
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => { 
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                },
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().val
        })
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
                    },
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
            println!("Dropping: {:}", node.borrow_mut().val);
        }
    }
}

fn main() {
    let mut dll = DoublyLinkedList::new();
    dll.insert_back(1);
    dll.insert_back(2);
    dll.insert_back(3);
    dll.insert_back(4);

    dll.traverse();

}
