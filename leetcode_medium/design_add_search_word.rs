use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            end_of_word: false,
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
    fn is_end_of_word(&self) -> bool {
        self.end_of_word
    }
    fn mark_end_of_word(&mut self) {
        self.end_of_word = true;
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

    fn insert(&mut self, s: String) {
        let mut curr = &mut self.root;
        for ch in s.chars() {
            if !curr.contains_key(&ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(&ch);
        }
        curr.mark_end_of_word();
    }

    fn search(&self, s: String) -> bool {
        self.search_helper(s, &self.root)
    }

    fn search_helper(&self, s: String, root: &Box<TrieNode>) -> bool {
        let mut curr = root;
        for i in 0..s.len() {
            let ch = *&s[i..i + 1].chars().nth(0).unwrap();
            if ch == '.' {
                // Match with every possible chars
                for (_, node) in curr.map.iter() {
                    if self.search_helper(String::from(&s[i + 1..]), node) {
                        return true;
                    }
                }
                return false;
            }
            if !curr.contains_key(&ch) {
                return false;
            }
            curr = curr.get_node(&ch);
        }
        curr.is_end_of_word()
    }
}

pub struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary { trie: Trie::new() }
    }

    fn add_word(&mut self, word: String) {
        self.trie.insert(word);
    }

    fn search(&self, word: String) -> bool {
        self.trie.search(word)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut wd = WordDictionary::new();
        wd.add_word(String::from("bad"));
        wd.add_word(String::from("dad"));
        wd.add_word(String::from("mad"));

        assert_eq!(wd.search(String::from("pad")), false);
        assert_eq!(wd.search(String::from("bad")), true);
        assert_eq!(wd.search(String::from(".ad")), true);
        assert_eq!(wd.search(String::from("b..")), true);
    }
}

fn main() {
    let mut wd = WordDictionary::new();
    wd.add_word(String::from("bad"));
    wd.add_word(String::from("dad"));
    wd.add_word(String::from("mad"));

    assert_eq!(wd.search(String::from("pad")), false);
    assert_eq!(wd.search(String::from("bad")), true);
    assert_eq!(wd.search(String::from(".ad")), true);
    assert_eq!(wd.search(String::from("b..")), true);
}
