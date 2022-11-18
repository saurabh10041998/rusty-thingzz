fn is_ugly(n: i32) -> bool {
    if n <= 0 { 
        return false;
    }
    let mut n = n;
    for p in vec![2,3,5] {
        while n % p == 0 { 
            n /= p;
        }
    }
    n == 1
}

#[cfg(test)]
pub mod tests {
    use crate::is_ugly;
    #[test]
    fn run_tc1() {
        let n = 6;
        assert_eq!(is_ugly(n), true);
    }
    #[test]
    fn run_tc2() {
        let n = 1;
        assert_eq!(is_ugly(n), true);
    }
    #[test]
    fn run_tc3() {
        let n = 14;
        assert_eq!(is_ugly(n), false);
    }

}

fn main() {
    println!("Hello, world!");
    let n = 14;
    assert_eq!(is_ugly(n), false);
}
