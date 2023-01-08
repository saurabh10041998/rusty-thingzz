use std::collections::HashMap;

pub struct TrieNode {
    mp: HashMap<char, Box<TrieNode>>,
    cnt_end: i32,
    cnt_prefix: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            cnt_end: 0,
            cnt_prefix: 0,
        }
    }

    fn contains_key(&self, ch: char) -> bool {
        match self.mp.get(&ch) {
            Some(_v) => true,
            None => false,
        }
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.mp.get(&ch) {
            Some(v) => v,
            None => {
                unreachable!()
            }
        }
    }

    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.mp.get_mut(&ch) {
            Some(v) => v,
            None => {
                unreachable!()
            }
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

    fn get_end_cnt(&self) -> i32 {
        self.cnt_end
    }

    fn get_prefix_cnt(&self) -> i32 {
        self.cnt_prefix
    }
}

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
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                curr.mp.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
            curr.increase_prefix();
        }
        curr.increase_end();
    }

    // Api to count the occurance of word in set
    fn count_word(&self, word: String) -> i32 {
        let mut curr = &self.root;
        for ch in word.chars() {
            if !curr.contains_key(ch) {
                return 0;
            }
            curr = curr.get_node(ch);
        }
        curr.get_end_cnt()
    }

    //Api to count the words starting with given prefix
    fn count_word_starts_with(&self, prefix: String) -> i32 {
        let mut curr = &self.root;
        for ch in prefix.chars() {
            if !curr.contains_key(ch) {
                return 0;
            }
            curr = curr.get_node(ch);
        }
        curr.get_prefix_cnt()
    }

    //Api to erase of word from the trie
    fn erase(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if curr.contains_key(ch) {
                curr = curr.get_node_mut(ch);
                curr.reduce_prefix();
            }else {
                unreachable!()
            }
        }
        curr.decrease_end();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc() {
        let mut trie = Trie::new();
        let dataset = vec![
            String::from("apple"),
            String::from("appl"),
            String::from("apps"),
            String::from("bac"),
            String::from("bat"),
        ];
        for d in dataset {
            trie.insert(d);
        }
        assert_eq!(trie.count_word(String::from("apple")), 1);
        assert_eq!(trie.count_word(String::from("appl")), 1);
        assert_eq!(trie.count_word(String::from("bax")), 0);

        assert_eq!(trie.count_word_starts_with(String::from("appl")), 2);
        assert_eq!(trie.count_word_starts_with(String::from("app")), 3);
        assert_eq!(trie.count_word_starts_with(String::from("ba")), 2);
        assert_eq!(trie.count_word_starts_with(String::from("bat")), 1);

        //trie.erase(String::from("xyz"));        // This will stop program from running
        trie.erase(String::from("appl"));   // OK
        assert_eq!(trie.count_word_starts_with(String::from("appl")), 1);
        assert_eq!(trie.count_word_starts_with(String::from("app")), 2);
        assert_eq!(trie.count_word_starts_with(String::from("ba")), 2);
        assert_eq!(trie.count_word_starts_with(String::from("bat")), 1);
    }
}
fn main() {
    let mut trie = Trie::new();
    let dataset = vec![
        String::from("apple"),
        String::from("appl"),
        String::from("apps"),
        String::from("bac"),
        String::from("bat"),
    ];
    for d in dataset {
        trie.insert(d);
    }
    assert_eq!(trie.count_word(String::from("apple")), 1);
    assert_eq!(trie.count_word(String::from("appl")), 1);
    assert_eq!(trie.count_word(String::from("bax")), 0);

    assert_eq!(trie.count_word_starts_with(String::from("appl")), 2);
    assert_eq!(trie.count_word_starts_with(String::from("app")), 3);
    assert_eq!(trie.count_word_starts_with(String::from("ba")), 2);
    assert_eq!(trie.count_word_starts_with(String::from("bat")), 1);

    //trie.erase(String::from("xyz"));        // This will stop program from running
    trie.erase(String::from("appl"));   // OK
    assert_eq!(trie.count_word_starts_with(String::from("appl")), 1);
    assert_eq!(trie.count_word_starts_with(String::from("app")), 2);
    assert_eq!(trie.count_word_starts_with(String::from("ba")), 2);
    assert_eq!(trie.count_word_starts_with(String::from("bat")), 1);
}
