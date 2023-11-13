use std::collections::BinaryHeap;
use std::cmp::Reverse; 

fn is_vowel(c: char) -> bool { 
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'O' | 'I' | 'U' => true,
        _ => false
    }
} 

fn sort_vowels(s: String) -> String {
    let mut pq: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    for c in s.chars() {
        if is_vowel(c) {
            pq.push(Reverse(c));
        }
    }
    
    let mut ans = String::new();
    for c in s.chars() { 
        if is_vowel(c) {
            match pq.pop() { 
                Some(Reverse(c)) => ans.push(c),
                None => unreachable!()
            };
        } else {
            ans.push(c);
        }

    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::sort_vowels;
    #[test]
    fn run_tc1() {
        let s = String::from("lEetcOde");
        assert_eq!(sort_vowels(s), String::from("lEOtcede"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("lYmpH");
        assert_eq!(sort_vowels(s), String::from("lYmpH"));
    }
}


fn main() {
    let s = String::from("lEetcOde");
    assert_eq!(sort_vowels(s), String::from("lEOtcede"));
}
