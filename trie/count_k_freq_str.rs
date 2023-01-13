use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct TrieNode {
    map: HashMap<char, Box<TrieNode>>,
    cnt_end: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            map: HashMap::new(),
            cnt_end: 0,
        }
    }
    fn contains_key(&self, ch: char) -> bool {
        self.map.contains_key(&ch)
    }
    fn get_node(&self, ch: char) -> &Box<TrieNode> {
        match self.map.get(&ch) {
            Some(v) => v,
            None => panic!("[+] Please check if key present before accessing map!!"),
        }
    }
    fn get_node_mut(&mut self, ch: char) -> &mut Box<TrieNode> {
        match self.map.get_mut(&ch) {
            Some(v) => v,
            None => panic!("Please check if key present before accessing map!!"),
        }
    }
    fn increase_end(&mut self) {
        self.cnt_end += 1;
    }
    fn get_end(&self) -> i32 {
        self.cnt_end
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
            if !curr.contains_key(ch) {
                curr.map.entry(ch).or_insert(Box::new(TrieNode::new()));
            }
            curr = curr.get_node_mut(ch);
        }
        curr.increase_end();
    }
    fn cnt_word(&self, word: &String) -> i32 {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Pair<'a> {
    cnt: i32,
    word: &'a str,
}

impl<'a> PartialOrd for Pair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cnt == other.cnt {
            return self.word.partial_cmp(other.word);
        }
        self.cnt.partial_cmp(&other.cnt)
    }
}

impl<'a> Ord for Pair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cnt == other.cnt {
            return self.word.cmp(other.word);
        }
        self.cnt.cmp(&other.cnt)
    }
}

fn find_k_frequent_node(buffer: Vec<String>, k: i32) -> Vec<String> {
    let mut trie = Trie::new();
    for w in buffer.iter() {
        trie.insert(w);
    }
    let mut pq = BinaryHeap::new();
    let mut query_set = HashSet::new();
    for w in buffer.iter() {
        let need_query = !query_set.contains(w);
        if need_query {
            let freq = trie.cnt_word(&w);
            query_set.insert(w.clone());
            pq.push(Pair {
                cnt: freq,
                word: w.as_str(),
            });
        }
    }
    let mut ans = vec![];
    let mut k = k;
    while let Some(p) = pq.pop() {
        ans.push(String::from(p.word));
        k -= 1;
        if k == 0 {
            break;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::find_k_frequent_node;
    #[test]
    fn run_tc1() {
        let buff_strs = [
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
        let mut buffer = vec![];
        for str in buff_strs {
            buffer.push(String::from(str));
        }
        let ans = vec![
            String::from("codec"),
            String::from("codecs"),
            String::from("coder"),
            String::from("code"),
        ];
        assert_eq!(find_k_frequent_node(buffer, 4), ans);
    }
}
fn main() {
    let buff_strs = [
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
    let mut buffer = vec![];
    for str in buff_strs {
        buffer.push(String::from(str));
    }
    let ans = vec![
        String::from("codec"),
        String::from("codecs"),
        String::from("coder"),
        String::from("code"),
    ];
    assert_eq!(find_k_frequent_node(buffer, 4), ans);
}
