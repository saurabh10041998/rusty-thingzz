fn reverse_parentheses(s: String) -> String {
    let mut s_new = s.clone();
    let mut buffer = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            buffer.push(i);
        } else if c == ')' {
            let revidx = buffer.last().unwrap();
            s_new = String::from(&s_new[..revidx + 1])
                + &s_new[*revidx + 1..i + 1].chars().rev().collect::<String>()
                + &s_new[i + 1..];
            buffer.pop();
        }
    }
    let mut ans = String::new();
    for c in s_new.chars() {
        if c != '(' && c != ')' {
            ans.push(c);
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("(abcd)");
        assert_eq!(reverse_parentheses(s), String::from("dcba"));
    }

    #[test]
    fn run_tc2() {
        let s = String::from("(u(love)i)");
        assert_eq!(reverse_parentheses(s), String::from("iloveu"));
    }

    #[test]
    fn run_tc3() {
        let s = String::from("(ed(et(oc))el)");
        assert_eq!(reverse_parentheses(s), String::from("leetcode"));
    }
}

fn main() {
    println!("Hello, world!");
    let s = String::from("(ed(et(oc))el)");
    assert_eq!(reverse_parentheses(s), String::from("leetcode"));
}
