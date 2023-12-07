fn largest_odd_number(num: String) -> String {
    num.chars()
        .rev()
        .skip_while(|&c| c as u8 & 1 == 0)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::largest_odd_number;
    #[test]
    fn run_tc1() {
        let s = String::from("52");
        assert_eq!(largest_odd_number(s), String::from("5"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("4206");
        assert_eq!(largest_odd_number(s), String::from(""));
    }
    #[test]
    fn run_tc3() {
        let s = String::from("35427");
        assert_eq!(largest_odd_number(s), String::from("35427"));
    }
}

fn main() {
    let s = String::from("35427");
    assert_eq!(largest_odd_number(s), String::from("35427"));
}
