fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = vec![0i32; n + 1]; 
    ans[0] = 0;
    for i in 1..n + 1 {
        ans[i] = ans[i / 2] + (i as i32 % 2);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::count_bits;
    #[test]
    fn run_tc1() {
        let n = 2;
        assert_eq!(count_bits(n), vec![0, 1, 1]);
    }
    #[test]
    fn run_tc2() {
        let n = 5;
        assert_eq!(count_bits(n), vec![0, 1, 1, 2, 1, 2]);
    }
}

fn main() {
    let n = 5;
    assert_eq!(count_bits(n), vec![0, 1, 1, 2, 1, 2]);
}