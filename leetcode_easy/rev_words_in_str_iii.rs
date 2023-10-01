use std::collections::VecDeque;

fn reverse_words(s: String) -> String {
    let mut q = VecDeque::new();
    let mut res = vec![];
    for x in s.chars() {
        if x != ' ' {
            q.push_front(x);
        } else {
            let mut tmp = String::new();
            while let Some(c) = q.pop_front() {
                tmp.push(c);
            }
            assert!(q.is_empty());
            res.push(tmp);
        }
    }
    let mut tmp = String::new();
    while let Some(c) = q.pop_front() {
        tmp.push(c);
    }
    res.push(tmp);

    res.join(" ")
}

#[cfg(test)]
pub mod tests {
    use crate::reverse_words;
    #[test]
    fn run_tc1() {
        let s = String::from("Let's take LeetCode contest");
        assert_eq!(
            reverse_words(s),
            String::from("s'teL ekat edoCteeL tsetnoc")
        );
    }
    #[test]
    fn run_tc2() {
        let s = String::from("God Ding");
        assert_eq!(reverse_words(s), String::from("doG gniD"));
    }
}

fn main() {
    let s = String::from("God Ding");
    assert_eq!(reverse_words(s), String::from("doG gniD"));
}
