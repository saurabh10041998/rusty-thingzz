use std::collections::HashMap;
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

    fn contains_key(&self, ch: char) -> bool {
        self.map.contains_key(&ch)
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.map.get(&ch) {
            Some(v) => v,
            None => panic!("{} character not found in map set", ch),
        }
    }

    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(v) => v,
            None => panic!("{} character not found in map set", ch),
        }
    }

    fn is_end_of_word(&self) -> bool {
        self.is_end
    }
    fn mark_end_of_word(&mut self) {
        self.is_end = true;
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

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
        }
        curr.mark_end_of_word();
    }

    fn word_break(&self, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        let n = word.len();
        let mut good = vec![false; n + 1];
        good[0] = true;
        for i in 0..n {
            if good[i] {
                let mut curr = &self.root;
                for j in i..n {
                    if !curr.contains_key(word[j]) {
                        break;
                    }
                    curr = curr.get_node(word[j]);
                    if curr.is_end_of_word() {
                        good[j + 1] = true;
                    }
                }
            }
        }
        good[n]
    }
}

fn solve(buffer: Vec<String>, word: String) -> bool {
    let mut trie = Trie::new();
    for w in buffer {
        trie.insert(w);
    }
    trie.word_break(word)
}

#[cfg(test)]
pub mod tests {
    use crate::solve;
    #[test]
    fn run_tc1() {
        let word_str = [
            "this", "th", "is", "famous", "word", "break", "b", "r", "e", "a", "k", "br", "bre",
            "brea", "ak", "prob", "lem",
        ];
        let mut buffer = vec![];
        for w in word_str {
            buffer.push(String::from(w));
        }
        let word = String::from("wordbreakproblem");
        assert_eq!(solve(buffer, word), true);
    }
}

fn main() {
    let word_str = [
        "this", "th", "is", "famous", "word", "break", "b", "r", "e", "a", "k", "br", "bre",
        "brea", "ak", "prob", "lem",
    ];
    let mut buffer = vec![];
    for w in word_str {
        buffer.push(String::from(w));
    }
    let word = String::from("wordbreakproblem");
    assert_eq!(solve(buffer, word), true);
}
