fn is_valid(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let mut st = vec![];
    for c in s {
        match c {
            '{' | '(' | '[' => {
                st.push(c);
            }
            '}' => {
                if let Some(t) = st.last() {
                    if *t == '{' {
                        st.pop().unwrap();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(t) = st.last() {
                    if *t == '[' {
                        st.pop().unwrap();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ')' => {
                if let Some(t) = st.last() {
                    if *t == '(' {
                        st.pop().unwrap();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    st.len() == 0
}

#[cfg(test)]
pub mod tests {
    use crate::is_valid;
    #[test]
    fn run_tc1() {
        let s = String::from("()");
        assert_eq!(is_valid(s), true);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("()[]{}");
        assert_eq!(is_valid(s), true);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("(}");
        assert_eq!(is_valid(s), false);
    }
    #[test]
    fn run_tc4() {
        let s = String::from("[");
        assert_eq!(is_valid(s), false);
    }
}

fn main() {
    let s = String::from("()[]{}");
    assert_eq!(is_valid(s), true);
}
