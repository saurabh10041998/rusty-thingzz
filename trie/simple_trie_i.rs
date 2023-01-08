use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    char_map: HashMap<char, Box<TrieNode>>,
    end_with: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            char_map: HashMap::new(),
            end_with: false,
        }
    }

    fn contains_key(&self, ch: char) -> bool {
        match self.char_map.get(&ch) {
            Some(_v) => true,
            None => false,
        }
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.char_map.get(&ch) {
            Some(v) => v,
            None => {
                unreachable!()
            }
        }
    }

    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.char_map.get_mut(&ch) {
            Some(node) => node,
            None => {
                unreachable!()
            }
        }
    }

    fn is_end_of_word(&self) -> bool {
        self.end_with
    }

    fn mark_end_of_word(&mut self) {
        self.end_with = true;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for w in word.chars() {
            if !curr.contains_key(w) {
                curr.char_map.entry(w).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(w);
        }
        curr.mark_end_of_word();
    }

    fn find_word(&self, word: String) -> bool {
        let mut curr = &self.root;
        for w in word.chars() {
            if !curr.contains_key(w) {
                return false;
            }
            curr = curr.get_node(w);
        }
        curr.is_end_of_word()
    }

    fn find_word_starting_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for w in prefix.chars() {
            if !curr.contains_key(w) {
                return false;
            }
            curr = curr.get_node(w);
        }
        return true;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut trie = Trie::new();
        trie.insert(String::from("apple"));
        trie.insert(String::from("appl"));
        trie.insert(String::from("apps"));
        trie.insert(String::from("bac"));
        trie.insert(String::from("bat"));

        assert_eq!(trie.find_word(String::from("apple")), true);
        assert_eq!(trie.find_word(String::from("app")), false);
        assert_eq!(trie.find_word(String::from("xyz")), false);
        assert_eq!(trie.find_word_starting_with(String::from("app")), true);
        assert_eq!(trie.find_word_starting_with(String::from("bax")), false);
        assert_eq!(trie.find_word_starting_with(String::from("zyyz")), false);
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert(String::from("apple"));
    trie.insert(String::from("appl"));
    trie.insert(String::from("apps"));
    trie.insert(String::from("bac"));
    trie.insert(String::from("bat"));

    assert_eq!(trie.find_word(String::from("apple")), true);
    assert_eq!(trie.find_word(String::from("app")), false);
    assert_eq!(trie.find_word(String::from("xyz")), false);
    assert_eq!(trie.find_word_starting_with(String::from("app")), true);
    assert_eq!(trie.find_word(String::from("bax")), false);
    assert_eq!(trie.find_word(String::from("zyyz")), false);
}
