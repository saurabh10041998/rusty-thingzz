fn bin_pow(mut x: f64, mut n: i64) -> f64 { 
    let mut ans:f64 = 1.0;
    while n != 0  {
        if n & 1 != 0 {
            ans *= x;
        }
        x *= x;
        n >>= 1;
    }
    ans
    
}

fn my_pow(x: f64, n: i32) -> f64 {
    let n: i64 = n as i64;
    if n < 0 {
        1.0 / bin_pow(x, n * -1)
    } else {
        bin_pow(x, n)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::my_pow;
    #[test]
    fn run_tc1() {
        let x = 2.0;
        let n = 10;
        assert_eq!(my_pow(x, n) as i64, 1024);
    }
    #[test]
    fn run_tc2() {
        let x = 2.10000;
        let n = 3;
        let ans = my_pow(x, n);
        println!("[*] Test 2: Actual ans = {}", ans);
        assert_eq!(ans as i64, 9);
    }
    #[test]
    fn run_tc3() {
        let x = 2.000;
        let n = -2;
        let ans = my_pow(x, n);
        println!("[*] Test 3: Actual ans = {}", ans);
        assert_eq!(ans as i64, 0);
    }
}

fn main() {
    let x = 2.000;
    let n = -2;
    let ans = my_pow(x, n);
    println!("[*] Test 3: Actual ans = {}", ans);
    assert_eq!(ans as i64, 0);
}