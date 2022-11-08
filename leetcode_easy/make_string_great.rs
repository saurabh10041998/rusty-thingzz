use std::ops::Sub;
fn abs_diff<T:PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x >= y {
        return x - y;
    }
    y - x
}

fn make_good(s: String) -> String {
    let mut arr = vec![];
    for c in s.chars() {
        if arr.last().is_some() && abs_diff(*arr.last().unwrap() as u8, c as u8) == 0x20 {
            arr.pop();
        } else {
            arr.push(c);
        }
    }
    arr.iter().collect::<String>()
}


// ################ Solution end #####################
// ################# Tests ########################
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s = String::from("leEeetcode");
        assert_eq!(make_good(s), "leetcode");
    }
    #[test]
    fn run_tc2() {
        let s = String::from("abBAcC");
        assert_eq!(make_good(s), "");
    }
}
fn main() {
    println!("Hello, world!");
    let s = String::from("abBAcC");
    assert_eq!(make_good(s), "");
}
