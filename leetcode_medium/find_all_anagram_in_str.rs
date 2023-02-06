use std::ops::Index;
use std::ops::IndexMut;

pub struct CharVec(Vec<i32>);

impl CharVec {
    fn new() -> Self {
        CharVec(vec![0; 26])
    }
}

impl Index<char> for CharVec {
    type Output = i32;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a'..='z' => &self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] Current character is not in charset"),
        }
    }
}

impl IndexMut<char> for CharVec {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a'..='z' => &mut self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] Current character is not in charset"),
        }
    }
}

impl PartialEq for CharVec {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for CharVec {}

fn find_anagram(s: String, p: String) -> Vec<i32> {
    let s_len = s.len();
    let p_len = p.len();
    let mut ans = vec![];
    if s_len < p_len {
        return ans;
    }
    let s = s.chars().collect::<Vec<char>>();
    let p = p.chars().collect::<Vec<char>>();
    let mut freq1 = CharVec::new();
    let mut freq2 = CharVec::new();
    for i in 0..p_len {
        freq1[s[i]] += 1;
        freq2[p[i]] += 1;
    }
    if freq1 == freq2 {
        ans.push(0);
    }
    for i in p_len..s_len {
        freq1[s[i - p_len]] -= 1;
        freq1[s[i]] += 1;
        if freq1 == freq2 {
            ans.push((i - p_len + 1) as i32);
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::find_anagram;
    #[test]
    fn run_tc1() {
        let s = String::from("cbaebabacd");
        let p = String::from("abc");
        assert_eq!(find_anagram(s, p), vec![0, 6]);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("abab");
        let p = String::from("ab");
        assert_eq!(find_anagram(s, p), vec![0, 1, 2]);
    }
}

fn main() {
    let s = String::from("cbaebabacd");
    let p = String::from("abc");
    assert_eq!(find_anagram(s, p), vec![0, 6]);
}
