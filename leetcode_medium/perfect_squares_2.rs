fn num_squares(n: i32) -> i32 {
    let mut n = n; 
    while n % 4 == 0 {
        n /= 4;
    }
    if n % 8 == 7 { 
        // number of form 4^k(8m + 7)
        return 4;
    }
    let mut x = 0; 
    while x * x <= n { 
        let y = f64::sqrt((n - x * x) as f64) as i32;
        if x * x  + y * y == n { 
            if x == 0 || y == 0 {
                return 1;
            }else {
                return 2;
            }
        }
        x += 1;
    }
    3
}

#[cfg(test)]
pub mod tests {
    use crate::num_squares;
    #[test]
    fn run_tc1() {
        let n = 12;
        assert_eq!(num_squares(n), 3);
    }
    #[test]
    fn run_tc2() {
        let n = 13;
        assert_eq!(num_squares(n), 2);
    }
}
fn main() {
    let n = 13;
    assert_eq!(num_squares(n), 2);
}
