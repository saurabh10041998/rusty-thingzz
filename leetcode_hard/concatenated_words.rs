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
            None => panic!("[!!] {} is not present in current charset", ch),
        }
    }
    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(node) => node,
            None => panic!("[!!] {} is not present in current charset", ch),
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
    fn insert(&mut self, word: &String) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            if !curr.contains_key(&ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
        }
        curr.mark_end_of_word();
    }
    fn check(&self, word: &String, word_idx: usize) -> bool {
        if word_idx == word.len() {
            return true;
        }
        let mut curr = &self.root;
        let mut flag = false;
        for curr_idx in word_idx..word.len() {
            if !curr.contains_key(&word[curr_idx..curr_idx + 1].chars().nth(0).unwrap()) {
                return false;
            }
            curr = curr.get_node(&word[curr_idx..curr_idx + 1].chars().nth(0).unwrap());
            if curr.is_end_of_word() {
                flag |= self.check(word, curr_idx + 1);
            }
            if flag {
                return flag;
            }
        }
        flag
    }
}

fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    let mut words = words;
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut ans = vec![];
    let mut trie = Trie::new();
    for c in words.iter() {
        if trie.check(c, 0) {
            ans.push(c.clone());
        }
        trie.insert(c);
    }
    ans
}
#[cfg(test)]
pub mod tests {
    use crate::find_all_concatenated_words_in_a_dict;
    #[test]
    fn run_tc1() {
        let words = vec![
            String::from("cat"),
            String::from("cats"),
            String::from("catsdogcats"),
            String::from("dog"),
            String::from("dogcatsdog"),
            String::from("hippopotamuses"),
            String::from("rat"),
            String::from("ratcatdogcat"),
        ];
        assert_eq!(
            find_all_concatenated_words_in_a_dict(words),
            vec![
                String::from("catsdogcats"),
                String::from("dogcatsdog"),
                String::from("ratcatdogcat")
            ]
        );
    }
    #[test]
    fn run_tc2() {
        let words = vec![
            String::from("cat"),
            String::from("dog"),
            String::from("catdog"),
        ];
        assert_eq!(
            find_all_concatenated_words_in_a_dict(words),
            vec![String::from("catdog")]
        );
    }
}
fn main() {
    let words = vec![
        String::from("cat"),
        String::from("cats"),
        String::from("catsdogcats"),
        String::from("dog"),
        String::from("dogcatsdog"),
        String::from("hippopotamuses"),
        String::from("rat"),
        String::from("ratcatdogcat"),
    ];
    assert_eq!(
        find_all_concatenated_words_in_a_dict(words),
        vec![
            String::from("catsdogcats"),
            String::from("dogcatsdog"),
            String::from("ratcatdogcat")
        ]
    );
}
