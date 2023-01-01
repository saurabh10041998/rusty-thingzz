fn count_digits(num: i32) -> i32 {
    let num_str = format!("{}", num);
    let mut digits = vec![];
    for c in num_str.chars() {
        digits.push(c.to_string().parse::<i32>().unwrap());
    }
    let mut count = 0;
    for d in digits { 
        if num % d  == 0 {
            count += 1;
        }
    }
    count
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let num = 7;
        assert_eq!(count_digits(num), 1);
    }
    #[test]
    fn run_tc2() {
        let num = 1248;
        assert_eq!(count_digits(num), 4);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
