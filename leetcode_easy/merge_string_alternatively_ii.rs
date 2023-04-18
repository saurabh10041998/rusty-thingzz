struct Zipper<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zipper<A, B> {
    fn new(a: A, b: B) -> Self {
        Zipper { a, b }
    }
}

impl<A, B> Iterator for Zipper<A, B>
where
    A: Iterator<Item = char>,
    B: Iterator<Item = char>,
{
    type Item = (Option<char>, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (Some(x), Some(y)) => Some((Some(x), Some(y))),
            (Some(x), None) => Some((Some(x), None)),
            (None, Some(y)) => Some((None, Some(y))),
            (None, None) => None,
        }
    }
}

fn merge_alternatively(word1: String, word2: String) -> String {
    let zipper = Zipper::new(word1.chars(), word2.chars());
    let mut res = String::new();
    for (x, y) in zipper {
        match x {
            Some(ch) => res.push(ch),
            None => {}
        };
        match y {
            Some(ch) => res.push(ch),
            None => {}
        };
    }
    res
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
