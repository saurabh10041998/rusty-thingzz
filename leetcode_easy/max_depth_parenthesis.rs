fn max_depth(s: String) -> i32 {
    s.chars()
        .fold((0, 0), |(max, count), ch| match ch {
            '(' => (i32::max(max, count + 1), count + 1),
            ')' => (max, count - 1),
            _ => (max, count),
        })
        .0
}

#[cfg(test)]
pub mod tests {
    use crate::max_depth;
    #[test]
    fn run_tc1() {
        let s = String::from("(1+(2*3)+((8)/4))+1");
        assert_eq!(max_depth(s), 3);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("(1)+((2))+(((3)))");
        assert_eq!(max_depth(s), 3);
    }
}

fn main() {
    let s = String::from("(1+(2*3)+((8)/4))+1");
    assert_eq!(max_depth(s), 3);
}
