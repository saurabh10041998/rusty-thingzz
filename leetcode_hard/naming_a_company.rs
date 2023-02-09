use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    is_start: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            is_start: false,
        }
    }

    fn contains_key(&self, ch: char) -> bool {
        self.map.contains_key(&ch)
    }

    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.map.get(&ch) {
            Some(node) => node,
            None => panic!("[+] Current character {} is not present in charmap", ch),
        }
    }

    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(node) => node,
            None => panic!("[+] Current character {} is not present in charmap", ch),
        }
    }

    fn is_start_of_word(&self) -> bool {
        self.is_start
    }

    fn mark_start_of_word(&mut self) {
        self.is_start = true;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    root: Box<TrieNode>,
    start: Vec<i32>, // record how many string start with this word
}

fn get_chr_idx(ch: char) -> usize {
    match ch {
        'a'..='z' => (ch as u8 - 'a' as u8) as usize,
        _ => panic!("[!!] Invalid character {} in given charset", ch),
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
            start: vec![0; 26],
        }
    }
    fn insert(&mut self, s: &String) {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut curr = &mut self.root;
        for i in (0..n).rev() {
            if !curr.contains_key(s[i]) {
                curr.map.entry(s[i]).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(s[i]);
        }
        self.start[get_chr_idx(s[0])] += 1;
        curr.mark_start_of_word();
    }

    fn traverse_suffix(&self, s: &String) -> Option<&Box<TrieNode>> {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut curr = &self.root;
        for i in (1..n).rev() {
            if !curr.contains_key(s[i]) {
                return None;
            }
            curr = curr.get_node(s[i]);
        }
        Some(curr)
    }
}

fn distinct_names(ideas: Vec<String>) -> i64 {
    let mut trie = Trie::new();
    for idea in ideas.iter() {
        trie.insert(idea);
    }

    let mut dp = vec![vec![0; 26]; 26];
    for c1 in 0..26 {
        for c2 in 0..26 {
            dp[c1][c2] = trie.start[c1];
        }
    }

    for idea in ideas.iter() {
        let curr = trie.traverse_suffix(idea);
        // SAFETY : Since its close trie on same dataset
        // safe to unwrap
        let curr = curr.unwrap();
        let c1 = get_chr_idx(*&idea[0..1].chars().nth(0).unwrap());
        for c2 in 'a'..='z' {
            if curr.contains_key(c2) && curr.get_node(c2).is_start_of_word() {
                dp[c1][get_chr_idx(c2)] -= 1;
            }
        }
    }

    let mut ans = 0;

    for idea in ideas.iter() {
        let curr = trie.traverse_suffix(idea);
        // SAFETY : Since its close trie on same dataset
        // safe to unwrap
        let curr = curr.unwrap();
        let c1 = get_chr_idx(*&idea[0..1].chars().nth(0).unwrap());
        for c2 in 'a'..='z' {
            if !curr.contains_key(c2) || !curr.get_node(c2).is_start_of_word() {
                ans += dp[get_chr_idx(c2)][c1] as i64;
            }
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::distinct_names;
    #[test]
    fn run_tc1() {
        let ideas = vec![
            String::from("coffee"),
            String::from("donuts"),
            String::from("time"),
            String::from("toffee"),
        ];
        assert_eq!(distinct_names(ideas), 6);
    }

    #[test]
    fn run_tc2() {
        let ideas = vec![String::from("lack"), String::from("back")];
        assert_eq!(distinct_names(ideas), 0);
    }
}

fn main() {
    let ideas = vec![
        String::from("coffee"),
        String::from("donuts"),
        String::from("time"),
        String::from("toffee"),
    ];
    assert_eq!(distinct_names(ideas), 6);
}
