use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    char_map: HashMap<char, Box<TrieNode>>,
    ends_with: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            char_map: HashMap::new(),
            ends_with: false,
        }
    }

    fn contains(&self, ch: char) -> bool {
        self.char_map.contains_key(&ch)
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.char_map.get(&ch) {
            Some(node) => node,
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
        self.ends_with
    }
    fn mark_end_of_word(&mut self) {
        self.ends_with = true;
    }
}

#[derive(Debug)]
struct Trie {
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
            if !curr.contains(ch) {
                curr.char_map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
        }
        curr.mark_end_of_word();
    }
    fn search(&self, s: String) -> String {
        let mut curr = &self.root;
        let mut j = 0;
        for ch in s.chars() {
            j += 1;
            if curr.is_end_of_word() {
                return String::from(&s[0..j - 1]);
            } else if !curr.contains(ch) {
                return s;
            } else {
                curr = curr.get_node(ch);
            }
        }
        s
    }
}

fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut tr = Trie::new();
    for dw in dictionary {
        tr.insert(dw);
    }
    let mut ans = vec![];
    for w in sentence.split_ascii_whitespace() {
        ans.push(tr.search(String::from(w)));
    }
    ans.join(" ")
}

#[cfg(test)]
pub mod tests {
    use crate::replace_words;
    #[test]
    fn run_tc1() {
        let dictionary = vec![
            String::from("cat"),
            String::from("bat"),
            String::from("rat"),
        ];
        let sentence = String::from("the cattle was rattled by the battery");
        assert_eq!(
            replace_words(dictionary, sentence),
            String::from("the cat was rat by the bat")
        );
    }
    #[test]
    fn run_tc2() {
        let dictionary = vec![String::from("a"), String::from("b"), String::from("c")];
        let sentence = String::from("aadsfasf absbs bbab cadsfafs");
        assert_eq!(replace_words(dictionary, sentence), String::from("a a b c"));
    }
}

fn main() {
    let dictionary = vec![
        String::from("cat"),
        String::from("bat"),
        String::from("rat"),
    ];
    let sentence = String::from("the cattle was rattled by the battery");
    assert_eq!(
        replace_words(dictionary, sentence),
        String::from("the cat was rat by the bat")
    );
}
