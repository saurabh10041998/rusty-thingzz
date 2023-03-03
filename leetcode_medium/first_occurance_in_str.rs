use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            is_end: false,
        }
    }
    fn contains_key(&self, ch: &char) -> bool {
        self.map.contains_key(ch)
    }
    fn get_node(&self, ch: &char) -> &Box<TrieNode> {
        match self.map.get(ch) {
            Some(subnode) => subnode,
            None => panic!("[!!] {} char not present in charmap", ch),
        }
    }
    fn get_node_mut(&mut self, ch: &char) -> &mut Box<TrieNode> {
        match self.map.get_mut(ch) {
            Some(subnode) => subnode,
            None => panic!("[!!] {} char not present in charmap", ch),
        }
    }
    fn is_end_of_word(&self) -> bool {
        self.is_end
    }
    fn mark_end_of_word(&mut self) {
        self.is_end = true;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    root: Box<TrieNode>,
}

impl<'a> Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
        }
    }
    fn insert(&mut self, s: &'a str) {
        let word = s.chars().collect::<Vec<char>>();
        let mut curr = &mut self.root;
        for &c in word.iter() {
            if !curr.contains_key(&c) {
                curr.map.entry(c).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(&c);
        }
        curr.mark_end_of_word();
    }

    // Solution..
    fn search(&self, s: &'a str) -> Option<usize> {
        let word = s.chars().collect::<Vec<char>>();
        let mut curr = &self.root;
        let mut ans = None;
        let mut i = 0;
        while i < word.len() {
            if curr.contains_key(&word[i]) {
                if ans.is_none() {
                    ans = Some(i);
                }
                curr = curr.get_node(&word[i]);
                if curr.is_end_of_word() {
                    return ans;
                }
            } else {
                match ans {
                    Some(j) => {
                        i = j;
                        ans = None;
                    }
                    None => ans = None,
                }
                curr = &self.root;
            }
            i += 1;
        }
        match curr.is_end_of_word() {
            true => ans,
            false => None,
        }
    }
}

// Main function
fn str_str(haystack: String, needle: String) -> i32 {
    let mut trie = Trie::new();
    trie.insert(needle.as_str());
    match trie.search(haystack.as_str()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::str_str;
    #[test]
    fn run_tc1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        assert_eq!(str_str(haystack, needle), 0);
    }
    #[test]
    fn run_tc2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        assert_eq!(str_str(haystack, needle), -1);
    }
}

fn main() {}
