use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<u8, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
        }
    }

    fn contains_key(&self, bit: u8) -> bool {
        self.map.contains_key(&bit)
    }

    fn get_node(&self, bit: u8) -> &Box<TrieNode> {
        match self.map.get(&bit) {
            Some(v) => v,
            None => panic!("Node not found for key {}", bit as i32),
        }
    }
    fn get_node_mut(&mut self, bit: u8) -> &mut Box<TrieNode> {
        match self.map.get_mut(&bit) {
            Some(v) => v,
            None => panic!("TrieNode not found for key {}", bit as i32),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
        }
    }
    fn insert(&mut self, num: i32) {
        let mut curr = &mut self.root;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as u8;
            if !curr.contains_key(bit) {
                curr.map.entry(bit).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(bit);
        }
    }

    fn get_max(&self, num: i32) -> i32 {
        let mut curr = &self.root;
        let mut max_num = 0;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as u8;
            if curr.contains_key(1 - bit) {
                max_num = max_num | (1 << i);
                curr = curr.get_node(1 - bit);
            } else {
                curr = curr.get_node(bit);
            }
        }
        max_num
    }

    fn print_trie(&self) {
        println!("{:#?}", self.root);
    }
}

fn get_max_xor(buffer1: Vec<i32>, buffer2: Vec<i32>) -> i32 {
    let mut trie = Trie::new();
    for &n in buffer1.iter() {
        trie.insert(n);
    }
    trie.print_trie();
    let mut maxi = 0;
    for &n in buffer2.iter() {
        maxi = i32::max(maxi, trie.get_max(n));
    }
    maxi
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer1 = vec![25, 10, 2, 8, 5, 3];
        let buffer2 = vec![25, 10, 2, 8, 5, 3];
        assert_eq!(get_max_xor(buffer1, buffer2), 28);
    }
}

fn main() {
    let buffer1 = vec![25, 10, 2, 8, 5, 3];
    let buffer2 = vec![25, 10, 2, 8, 5, 3];
    assert_eq!(get_max_xor(buffer1, buffer2), 28);
}
