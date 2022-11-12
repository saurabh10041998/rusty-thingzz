fn is_happy(n: i32) -> bool {
    // 1 and 7 leads to happy number only
    let (mut sum, mut x) = (n, n);
    while sum > 9 {
        sum = 0;
        while x != 0 {
            let d = x % 10;
            sum += d * d;
            x /= 10;
        }
        if sum == 1 {
            return true;
        }
        x = sum;
    }
    match sum {
        1 => true,
        7 => true,
        _ => false,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::is_happy;
    #[test]
    fn run_tc1() {
        let n = 19;
        assert_eq!(is_happy(n), true);
    }

    #[test]
    fn run_tc2() {
        let n = 2;
        assert_eq!(is_happy(n), false);
    }

    #[test]
    fn run_tc3() {
        let n = 1;
        assert_eq!(is_happy(n), true);
    }
}

fn main() {
    println!("Hello, world!");
    let n = 2;
    assert_eq!(is_happy(n), false);
}
