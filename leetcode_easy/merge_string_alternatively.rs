fn merge_alternatively(word1: String, word2: String) -> String {
    let mut w1_iter = word1.chars().into_iter().peekable();
    let mut w2_iter = word2.chars().into_iter().peekable();
    let mut ans = String::new();
    loop {
        if let Some(c1) = w1_iter.next() {
            ans.push(c1);
        }
        if let Some(c2) = w2_iter.next() {
            ans.push(c2);
        }
        if w1_iter.peek().is_none() && w2_iter.peek().is_none() {
            return ans;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::merge_alternatively;
    #[test]
    fn run_tc1() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        assert_eq!(merge_alternatively(word1, word2), String::from("apbqcr"));
    }
    #[test]
    fn run_tc2() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        assert_eq!(merge_alternatively(word1, word2), String::from("apbqrs"));
    }
    #[test]
    fn run_tc3() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        assert_eq!(merge_alternatively(word1, word2), String::from("apbqcd"));
    }
}

fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    assert_eq!(merge_alternatively(word1, word2), String::from("apbqcr"));    
}
