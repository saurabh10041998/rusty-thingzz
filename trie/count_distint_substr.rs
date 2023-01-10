use std::collections::HashMap; 

pub struct TrieNode { 
    map: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new()
        }
    }
    fn contains_key(&self, ch: char) -> bool {
        self.map.contains_key(&ch)
    }
   
    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(v) => {
                v
            },
            None => {
                panic!("Implementation error: call contains key before accessing node");
            }
        }
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
    fn count_distint_substrings(&mut self, word: String) -> i32 {        
        let n = word.len();
        let mut cnt = 0;
        for i in 0..n {
            let mut curr = &mut self.root;
            for j in i..n {
                let key = &word[j..j+1].chars().nth(0).unwrap();
                if !curr.contains_key(*key) {
                    curr.map.entry(*key).or_insert(Box::new(TrieNode::new()));
                    cnt += 1;
                }
                curr = curr.get_node_mut(*key);
            }
        }
        cnt + 1
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut trie = Trie::new();
        assert_eq!(trie.count_distint_substrings(String::from("abab")), 8);
    }
}

fn main() {
    let mut trie = Trie::new();
    assert_eq!(trie.count_distint_substrings(String::from("abab")), 8);
}