fn remove_duplicates(s: String) -> String {
    let mut ans = vec![];
    for c in s.chars() {
        if ans.last().is_some() && ans.last().unwrap() == &c {
            ans.pop();
        }else {
            ans.push(c);
        }
    }
    ans.iter().collect::<String>()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("abbaca");
        assert_eq!(remove_duplicates(s), String::from("ca"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("azxxzy");
        assert_eq!(remove_duplicates(s), String::from("ay"));
    }
}

fn main() {
    let s = String::from("azxxzy");
    assert_eq!(remove_duplicates(s), String::from("ay"));
}