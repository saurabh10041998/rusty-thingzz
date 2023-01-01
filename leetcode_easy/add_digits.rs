fn add_digits(num: i32) -> i32 {
    let mut num_str = format!("{}", num);
    while num_str.len() != 1 {
        let sum: i32 = num_str
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .sum();
        num_str = format!("{}", sum);
    }
    num_str.parse::<i32>().unwrap()
}
#[cfg(test)]
pub mod tests {
    use crate::add_digits;
    #[test]
    fn run_tc1() {
        let num = 38;
        assert_eq!(add_digits(num), 2);
    }
    #[test]
    fn run_tc2() {
        let num = 0;
        assert_eq!(add_digits(num), 0);
    }
}
fn main() {
    let num = 38;
    assert_eq!(add_digits(num), 2);
}
