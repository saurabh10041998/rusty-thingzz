fn decode_at_index(s: String, k: i32) -> String {
    let mut size: usize = 0;
    let mut k = k as usize;
    for c in s.chars() {
        match c.to_string().parse::<usize>() {
            Ok(val) => {
                size *= val;
            }
            Err(_) => {
                size += 1;
            }
        }
    }
    for c in s.chars().rev() {
        match c.to_string().parse::<usize>() {
            Ok(val) => {
                size /= val;
                k %= size;
            }
            Err(_) => {
                if k == 0 || k == size {
                    return String::from(c);
                }
                size -= 1;
            }
        }
    }
    String::from("")
}

#[cfg(test)]
pub mod tests {
    use crate::decode_at_index;
    #[test]
    fn run_tc1() {
        let s = String::from("leet2code3");
        let k = 10;
        assert_eq!(decode_at_index(s, k), String::from("o"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("ha22");
        let k = 5;
        assert_eq!(decode_at_index(s, k), String::from("h"));
    }
    #[test]
    fn run_tc3() {
        let s = String::from("a2345678999999999999999");
        let k = 1;
        assert_eq!(decode_at_index(s, k), String::from("a"));
    }
}
fn main() {
    let s = String::from("a2345678999999999999999");
    let k = 1;
    assert_eq!(decode_at_index(s, k), String::from("a"));
}
