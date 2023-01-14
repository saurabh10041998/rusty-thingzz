use std::ops::Index;
use std::ops::IndexMut;
pub struct CharVec(Vec<char>);

impl CharVec {
    fn new() -> Self {
        CharVec(('a'..='z').collect::<Vec<char>>())
    }
}

impl Index<char> for CharVec {
    type Output = char;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a'..='z' => &self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] {} character is not support in current charset", index),
        }
    }
}

impl IndexMut<char> for CharVec {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a'..='z' => &mut self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] {} character is not support in current charset", index),
        }
    }
}

pub struct DisjointSet {
    parent: CharVec,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            parent: CharVec::new(),
        }
    }

    fn find(&mut self, ch: char) -> char {
        if self.parent[ch] == ch {
            return ch;
        }
        self.parent[ch] = self.find(self.parent[ch]);
        self.parent[ch]
    }

    fn union(&mut self, x: char, y: char) {
        let x_par = self.find(x);
        let y_par = self.find(y);

        if x_par != y_par {
            self.parent[char::max(x_par, y_par)] = char::min(x_par, y_par);
        }
    }
}

fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut ds = DisjointSet::new();
    for (x, y) in s1.chars().zip(s2.chars()) {
        ds.union(x, y);
    }
    let mut ans = String::new();
    for c in base_str.chars() {
        ans.push(ds.find(c));
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::smallest_equivalent_string;
    #[test]
    fn run_tc1() {
        let (s1, s2) = (String::from("parker"), String::from("morris"));
        let base_str = String::from("parser");
        assert_eq!(
            smallest_equivalent_string(s1, s2, base_str),
            String::from("makkek")
        );
    }
    #[test]
    fn run_tc2() {
        let (s1, s2) = (String::from("hello"), String::from("world"));
        let base_str = String::from("hold");
        assert_eq!(
            smallest_equivalent_string(s1, s2, base_str),
            String::from("hdld")
        );
    }
    #[test]
    fn run_tc3() {
        let (s1, s2) = (String::from("leetcode"), String::from("programs"));
        let base_str = String::from("sourcecode");
        assert_eq!(
            smallest_equivalent_string(s1, s2, base_str),
            String::from("aauaaaaada")
        );
    }
}

fn main() {
    let (s1, s2) = (String::from("parker"), String::from("morris"));
    let base_str = String::from("parser");
    assert_eq!(
        smallest_equivalent_string(s1, s2, base_str),
        String::from("makkek")
    );
}
