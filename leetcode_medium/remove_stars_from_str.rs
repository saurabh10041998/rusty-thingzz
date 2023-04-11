fn remove_stars(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let mut stack = vec![];
    for c in s {
        match c {
            '*' => match stack.pop() {
                Some(_c) => {}
                None => {}
            },
            _ => {
                stack.push(c);
            }
        }
    }
    stack.into_iter().collect::<String>()
}

#[cfg(test)]
pub mod tests {
    use crate::remove_stars;
    #[test]
    fn run_tc1() {
        let s = String::from("leet**cod*e");
        assert_eq!(remove_stars(s), String::from("lecoe"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("erase*****");
        assert_eq!(remove_stars(s), String::from(""));
    }
}

fn main() {
    let s = String::from("leet**cod*e");
    assert_eq!(remove_stars(s), String::from("lecoe"));
}
