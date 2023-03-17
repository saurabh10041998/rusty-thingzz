use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    cnt_prefix: i32,
    cnt_end: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            cnt_prefix: 0,
            cnt_end: 0,
        }
    }
    fn contains_key(&self, ch: &char) -> bool {
        self.map.contains_key(ch)
    }
    fn get_node(&self, ch: &char) -> &Box<TrieNode> {
        match self.map.get(ch) {
            Some(node) => node,
            None => panic!("[!!] Given character {} is not present in charmap", ch),
        }
    }
    fn get_node_mut(&mut self, ch: &char) -> &mut Box<TrieNode> {
        match self.map.get_mut(ch) {
            Some(node) => node,
            None => panic!("[!!] Given character {} is not present in charmap", ch),
        }
    }
    fn increase_end(&mut self) {
        self.cnt_end += 1;
    }
    fn increase_prefix(&mut self) {
        self.cnt_prefix += 1;
    }
    fn reduce_prefix(&mut self) {
        self.cnt_prefix -= 1;
    }
    fn decrease_end(&mut self) {
        self.cnt_end -= 1;
    }
    fn get_cnt(&self) -> i32 {
        self.cnt_end
    }
    fn get_prefix(&self) -> i32 {
        self.cnt_prefix
    }
}

pub struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
        }
    }

    // Api to insert string in trie
    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if !curr.contains_key(&ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(&ch);
            curr.increase_prefix();
        }
        curr.increase_end();
    }

    //Api to search the word
    fn search(&self, word: String) -> bool {
        let mut curr = &self.root;
        for ch in word.chars() {
            if !curr.contains_key(&ch) {
                return false;
            }
            curr = curr.get_node(&ch);
        }
        curr.get_cnt() > 0
    }

    //Api to search the prefix
    fn starts_with(&self, word: String) -> bool {
        let mut curr = &self.root;
        for ch in word.chars() {
            if !curr.contains_key(&ch) {
                return false;
            }
            curr = curr.get_node(&ch);
        }
        curr.get_prefix() > 0
    }

    //Api to erase the word from trie
    #[allow(dead_code)]
    fn erase(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if curr.contains_key(&ch) {
                curr = curr.get_node_mut(&ch);
                curr.reduce_prefix();
            } else {
                unreachable!()
            }
        }
        curr.decrease_end()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut trie = Trie::new();
        trie.insert(String::from("apple"));
        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), false);
        assert_eq!(trie.starts_with(String::from("app")), true);
        trie.insert(String::from("app"));
        assert_eq!(trie.search(String::from("app")), true);
    }
}
fn main() {
    let mut trie = Trie::new();
    trie.insert(String::from("apple"));
    assert_eq!(trie.search(String::from("apple")), true);
    assert_eq!(trie.search(String::from("app")), false);
    assert_eq!(trie.starts_with(String::from("app")), true);
    trie.insert(String::from("app"));
    assert_eq!(trie.search(String::from("app")), true);
}
