use std::ops::{Index, IndexMut};

struct CharVec {
    count: Vec<usize>,
    is_empty: bool,
}

impl Index<char> for CharVec {
    type Output = usize;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a'..='z' => &self.count[(index as u8 - 'a' as u8) as usize],
            _ => panic!("{} character is not supported in current charset", index),
        }
    }
}

impl IndexMut<char> for CharVec {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a'..='z' => &mut self.count[(index as u8 - 'a' as u8) as usize],
            _ => panic!("{} character is not supported in current charset", index),
        }
    }
}

impl CharVec {
    fn new() -> Self {
        CharVec {
            count: vec![0usize; 26],
            is_empty: false,
        }
    }
    fn update(&mut self, s: String) {
        for c in s.chars() {
            self[c] += 1;
        }
    }
    fn intersection(self, other: Self) -> Self {
        let mut modified = self;
        for (a, b) in modified.count.iter_mut().zip(other.count.iter()) {
            *a = usize::min(*a, *b);
            modified.is_empty &= *a == 0;
        }
        modified
    }
    fn to_string_vec(&self) -> Vec<String> {
        let mut ans = vec![];
        let alpha = ('a'..='z').collect::<String>();
        for c in alpha.chars() {
            for _ in 0..self[c] {
                ans.push(c.to_string())
            }
        }
        ans
    }
}

fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut s = CharVec::new();
    s.update(words[0].clone());
    for word in words.into_iter().skip(1) {
        let mut other = CharVec::new();
        other.update(word);
        s = s.intersection(other);
    }
    s.to_string_vec()
}

#[cfg(test)]
pub mod tests {
    use crate::common_chars;
    #[test]
    fn run_tc1() {
        let words = vec![
            String::from("bella"),
            String::from("label"),
            String::from("roller"),
        ];
        assert_eq!(
            common_chars(words),
            vec![String::from("e"), String::from("l"), String::from("l")]
        );
    }
    #[test]
    fn run_tc2() {
        let words = vec![
            String::from("cool"),
            String::from("lock"),
            String::from("cook"),
        ];
        assert_eq!(
            common_chars(words),
            vec![String::from("c"), String::from("o")]
        );
    }
}

fn main() {
    let words = vec![
        String::from("bella"),
        String::from("label"),
        String::from("roller"),
    ];
    assert_eq!(
        common_chars(words),
        vec![String::from("e"), String::from("l"), String::from("l")]
    );
}
