use std::cmp::Ordering;
use std::collections::HashMap;

pub struct AlienDictionary {
    order_map: HashMap<char, usize>,
}

impl<'a> AlienDictionary {
    fn new(order: &'a str) -> Self {
        let mut order_map = HashMap::new();
        for (i, c) in order.chars().enumerate() {
            order_map.entry(c).or_insert(i);
        }
        AlienDictionary { order_map }
    }
    fn get_char_idx(&self, ch: char) -> usize {
        match self.order_map.get(&ch) {
            Some(idx) => *idx,
            None => panic!("[!!] Current character not in dictionary"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct AlienWord<'a> {
    word: String,
    order: &'a str,
}

impl<'a> AlienWord<'a> {
    fn new(s: String, order: &'a str) -> Self {
        AlienWord { word: s, order }
    }
}

impl<'a> PartialOrd for AlienWord<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let dictionary = AlienDictionary::new(self.order);
        let a = self.word.chars().collect::<Vec<char>>();
        let b = other.word.chars().collect::<Vec<char>>();
        let n = usize::min(a.len(), b.len());
        for i in 0..n {
            let x = dictionary.get_char_idx(a[i]);
            let y = dictionary.get_char_idx(b[i]);
            if x > y {
                return Some(Ordering::Greater);
            }
            if x < y {
                return Some(Ordering::Less);
            }
        }
        a.len().partial_cmp(&b.len())
    }
}

impl<'a> Ord for AlienWord<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let dictionary = AlienDictionary::new(self.order);
        let a = self.word.chars().collect::<Vec<char>>();
        let b = other.word.chars().collect::<Vec<char>>();
        let n = usize::min(a.len(), b.len());
        for i in 0..n {
            let x = dictionary.get_char_idx(a[i]);
            let y = dictionary.get_char_idx(b[i]);
            if x > y {
                return Ordering::Greater;
            }
            if x < y {
                return Ordering::Less;
            }
        }
        a.len().cmp(&b.len())
    }
}

fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut alien_words = vec![];
    for w in words.iter() {
        alien_words.push(AlienWord::new(w.clone(), order.as_str()));
    }
    alien_words.sort();
    let mut words_copy = vec![];
    for a_w in alien_words {
        words_copy.push(a_w.word);
    }
    words_copy == words
}

#[cfg(test)]
pub mod tests {
    use crate::is_alien_sorted;
    #[test]
    fn run_tc1() {
        let words = vec![String::from("hello"), String::from("leetcode")];
        let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
        assert_eq!(is_alien_sorted(words, order), true);
    }

    #[test]
    fn run_tc2() {
        let words = vec![
            String::from("word"),
            String::from("world"),
            String::from("row"),
        ];
        let order = String::from("worldabcefghijkmnpqstuvxyz");
        assert_eq!(is_alien_sorted(words, order), false);
    }

    #[test]
    fn run_tc3() {
        let words = vec![String::from("apple"), String::from("app")];
        let order = String::from("abcdefghijklmnopqrstuvwxyz");
        assert_eq!(is_alien_sorted(words, order), false);
    }
}
fn main() {
    let words = vec![String::from("hello"), String::from("leetcode")];
    let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
    assert_eq!(is_alien_sorted(words, order), true);
}
