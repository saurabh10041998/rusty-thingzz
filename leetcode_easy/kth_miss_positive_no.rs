fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut mp = vec![false; 2001];
    arr.iter().for_each(|x| mp[*x as usize] = true);
    let mut cnt = 0;
    for (i, m) in mp.iter().enumerate() {
        if i == 0 {
            continue; // Expected positive numbers only
        }
        match *m {
            true => {}
            false => {
                cnt += 1;
                if cnt == k {
                    return i as i32;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::find_kth_positive;
    #[test]
    fn run_tc1() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(find_kth_positive(arr, k), 9);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![1, 2, 3, 4];
        let k = 2;
        assert_eq!(find_kth_positive(arr, k), 6);
    }
}

fn main() {
    let arr = vec![2, 3, 4, 7, 11];
    let k = 5;
    assert_eq!(find_kth_positive(arr, k), 9);
}
