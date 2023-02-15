fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    // Vector to store the result of operation
    let mut res = vec![];
    // Make k mutable
    let mut k = k;
    for n in num.iter().rev() {
        let x = *n + k;
        res.push(x % 10);
        k = x / 10;
    }
    while k != 0 {
        res.push(k % 10);
        k /= 10;
    }
    res.iter().rev().map(|x| *x).collect()
}

#[cfg(test)]
pub mod tests {
    use crate::add_to_array_form;
    #[test]
    fn run_tc1() {
        let num = vec![1, 2, 0, 0];
        let k = 34;
        assert_eq!(add_to_array_form(num, k), vec![1, 2, 3, 4]);
    }

    #[test]
    fn run_tc2() {
        let num = vec![2, 7, 4];
        let k = 181;
        assert_eq!(add_to_array_form(num, k), vec![4, 5, 5]);
    }
    #[test]
    fn run_tc3() {
        let num = vec![2, 1, 5];
        let k = 806;
        assert_eq!(add_to_array_form(num, k), vec![1, 0, 2, 1]);
    }
    #[test]
    fn run_tc4() {
        let num = vec![0];
        let k = 10000;
        assert_eq!(add_to_array_form(num, k), vec![1, 0, 0, 0, 0]);
    }
}

fn main() {
    let num = vec![1, 2, 0, 0];
    let k = 34;
    assert_eq!(add_to_array_form(num, k), vec![1, 2, 3, 4]);    
}
