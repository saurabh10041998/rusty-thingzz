#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

const MAX_SIZE: usize = usize::pow(10, 4);

struct MyHashSet {
    set: Vec<Box<ListNode>>,
}

impl MyHashSet {
    fn new() -> Self {
        let mut set = vec![];
        set.resize_with(MAX_SIZE, || Box::new(ListNode::new(-1)));
        MyHashSet { set }
    }
    fn add(&mut self, key: i32) {
        let mut curr = &mut self.set[key as usize % MAX_SIZE];
        while curr.next.is_some() {
            if curr.next.as_ref().unwrap().val == key {
                return;
            }
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(Box::new(ListNode::new(key)));
    }

    fn contains(&self, key: i32) -> bool {
        let mut curr = &self.set[key as usize % MAX_SIZE];
        while curr.next.is_some() {
            if curr.next.as_ref().unwrap().val == key {
                return true;
            }
            curr = curr.next.as_ref().unwrap();
        }
        false
    }

    fn remove(&mut self, key: i32) {
        let mut curr = &mut self.set[key as usize % MAX_SIZE];
        while curr.next.is_some() {
            if curr.next.as_ref().unwrap().val == key {
                let mut del_node = curr.next.take();
                curr.next = del_node.as_mut().unwrap().next.take();
                drop(del_node);
                return;
            }
            curr = curr.next.as_mut().unwrap();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert_eq!(set.contains(1), true);
        assert_eq!(set.contains(3), false);
        set.add(2);
        assert_eq!(set.contains(2), true);
        set.remove(2);
        assert_eq!(set.contains(2), false);
    }
}

fn main() {
    let mut set = MyHashSet::new();
    set.add(1);
    set.add(2);
    assert_eq!(set.contains(1), true);
    assert_eq!(set.contains(3), false);
    set.add(2);
    assert_eq!(set.contains(2), true);
    set.remove(2);
    assert_eq!(set.contains(2), false);
}
