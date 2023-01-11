use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    cnt_end: i32,
    cnt_prefix: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            cnt_end: 0,
            cnt_prefix: 0,
        }
    }
    fn contains_key(&self, ch: char) -> bool {
        self.map.contains_key(&ch)
    }
    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.map.get(&ch) {
            Some(node) => node,
            None => panic!("Node not found"),
        }
    }
    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(node) => node,
            None => panic!("Node not found"),
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
    fn get_end(&self) -> i32 {
        self.cnt_end
    }
    fn get_prefix(&self) -> i32 {
        self.cnt_prefix
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
            curr.increase_prefix();
        }
        curr.increase_end();
    }
    fn cnt_word(&self, word: String) -> i32 {
        let mut curr = &self.root;
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                return 0;
            }
            curr = curr.get_node(ch);
        }
        curr.get_end()
    }
}

fn max_occurance_word(keys: Vec<String>) -> (i32, Option<String>) {
    let mut trie = Trie::new();
    for c in keys.iter() {
        trie.insert(c.clone());
    }
    let mut max_count = 0;
    let mut key = String::new();
    for c in keys {
        let freq = trie.cnt_word(c.clone());
        if freq > max_count {
            max_count = freq;
            key = c;
        }
    }
    match max_count {
        0 => (0, None),
        val => (val, Some(key)),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::max_occurance_word;
    #[test]
    fn run_tc1() {
        let raw_keys = [
            "code",
            "coder",
            "coding",
            "codable",
            "codec",
            "codecs",
            "coded",
            "codeless",
            "codec",
            "codecs",
            "codependence",
            "codex",
            "codify",
            "codependents",
            "codes",
            "code",
            "coder",
            "codesign",
            "codec",
            "codeveloper",
            "codrive",
            "codec",
            "codecs",
            "codiscovered",
        ];
        let mut keys = vec![];
        for k in raw_keys {
            keys.push(String::from(k));
        }
        assert_eq!(max_occurance_word(keys), (4, Some(String::from("codec"))));
    }
}

fn main() {
    let raw_keys = [
        "code",
        "coder",
        "coding",
        "codable",
        "codec",
        "codecs",
        "coded",
        "codeless",
        "codec",
        "codecs",
        "codependence",
        "codex",
        "codify",
        "codependents",
        "codes",
        "code",
        "coder",
        "codesign",
        "codec",
        "codeveloper",
        "codrive",
        "codec",
        "codecs",
        "codiscovered",
    ];
    let mut keys = vec![];
    for k in raw_keys {
        keys.push(String::from(k));
    }
    assert_eq!(max_occurance_word(keys), (4, Some(String::from("codec"))))
}
