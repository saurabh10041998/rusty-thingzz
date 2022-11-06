fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    let (mut start, mut end, mut ans) = (1, x / 2, x);
    while start <= end {
        let mid = (start + end) / 2;
        let sqr = mid as i64 * mid as i64;
        // in case, x is perfect square.
        if sqr == x  as i64 {
            return mid;
        }
        if sqr  <= x as i64 {
            start = mid + 1;
            ans = mid;
        } else {
            end = mid - 1;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let x = 4;
        assert_eq!(2, my_sqrt(x));
    }

    #[test]
    fn run_tc2() {
        let x = 8;
        assert_eq!(2, my_sqrt(x));
    }

    #[test]
    fn run_tc3() {
        let x = 2147395599;
        assert_eq!(46339, my_sqrt(x));
    }
}
fn main() {
    println!("Hello, world!");
    let x = 8;
    assert_eq!(2, my_sqrt(x));
}
