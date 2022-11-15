fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return Vec::<String>::new();
    }
    let (mut start, mut end) = (-1, -1);
    let mut ans = Vec::new();
    for (i, c) in nums.into_iter().enumerate() {
        if i == 0 {
            start = c;
            end = c;
            continue;
        }
        else if c == end + 1 {
            end = c;
        }else {
            if start == end {
                ans.push(format!("{}", start));
            }else {
                ans.push(format!("{}->{}", start, end));
            }
            start = c;
            end = c;
        }
    }
    if start == end {
        ans.push(format!("{}", start));
    }else {
        ans.push(format!("{}->{}", start, end));
    }

    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![0,1,2,4,5,7];
        assert_eq!(summary_ranges(nums), vec![String::from("0->2"), String::from("4->5"), String::from("7")]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0,2,3,4,6,8,9];
        assert_eq!(summary_ranges(nums), vec![String::from("0"), String::from("2->4"), String::from("6"), String::from("8->9")]);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![];
        assert_eq!(summary_ranges(nums), Vec::<String>::new());
    }
    #[test]
    fn run_tc4() {
        let nums = vec![-1];
        assert_eq!(summary_ranges(nums), vec![String::from("-1")]);
    }
}


fn main() {
    println!("Hello, world!");
    let nums = vec![0,2,3,4,6,8,9];
    assert_eq!(summary_ranges(nums), vec![String::from("0"), String::from("2->4"), String::from("6"), String::from("8->9")]);
}
