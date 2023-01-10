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

    fn contains_key(&self, ch: char) -> bool {
        match self.map.get(&ch) {
            Some(_v) => true,
            None => false,
        }
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.map.get(&ch) {
            Some(v) => v,
            None => {
                panic!("Check if {} exists with contains key before get_node", ch);
            }
        }
    }

    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(v) => v,
            None => {
                panic!(
                    "Check if {} exists with contains key before get_node_mut",
                    ch
                );
            }
        }
    }

    fn is_end_of_word(&self) -> bool {
        self.end_of_word
    }

    fn set_end_of_word(&mut self) {
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

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
        }
        curr.set_end_of_word();
    }

    fn check_prefix_present(&self, word: String) -> bool {
        let mut curr = &self.root;
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                return false;
            }
            curr = curr.get_node(ch);
            if !curr.is_end_of_word() {
                return false;
            }
        }
        true
    }
}

fn longest_complete_string(haystacks: Vec<String>) -> Option<String> {
    let mut trie = Trie::new();
    for s in haystacks.iter() {
        trie.insert(String::from(s.as_str()));
    }
    let mut longest = String::new();
    for s in haystacks.iter() {
        if trie.check_prefix_present(String::from(s)) {
            if s.len() > longest.len() {
                longest = String::from(s);
            }
            if s.len() == longest.len() && &longest[..] > s {
                longest = String::from(s);
            }
        }
    }
    if longest.len() == 0 {
        return None;
    }
    Some(longest)
}

#[cfg(test)]
pub mod tests {
    use crate::longest_complete_string;
    #[test]
    fn run_tc1() {
        let haystacks = vec![
            String::from("n"),
            String::from("ni"),
            String::from("nin"),
            String::from("ninja"),
            String::from("ninj"),
            String::from("ninga"),
        ];
        assert_eq!(
            longest_complete_string(haystacks),
            Some(String::from("ninja"))
        );
    }
    #[test]
    fn run_tc2() {
        let haystacks = vec![
            String::from("random3"),
            String::from("random1"),
            String::from("rando2"),
        ];
        assert_eq!(longest_complete_string(haystacks), None);
    }
}

fn main() {
    let haystacks = vec![
        String::from("n"),
        String::from("ni"),
        String::from("nin"),
        String::from("ninja"),
        String::from("ninj"),
        String::from("ninga"),
    ];
    assert_eq!(
        longest_complete_string(haystacks),
        Some(String::from("ninja"))
    );
}
