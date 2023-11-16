use std::collections::HashSet;

fn int_to_binary_string(number: i32, length: usize) -> String {
    let binary_representation = format!("{:b}", number);
    if binary_representation.len() < length {
        let padding = "0".repeat(length - binary_representation.len());
        format!("{}{}", padding, binary_representation)
    } else if binary_representation.len() > length {
        binary_representation
            .chars()
            .skip(binary_representation.len() - length)
            .collect()
    } else {
        binary_representation
    }
}

fn find_different_binary_string(mut nums: Vec<String>) -> String {
    let n = nums.len();
    let string_set: HashSet<String> = nums.drain(..).collect();
    for i in 0..=n {
        let binstr = int_to_binary_string(i as i32, n);
        if !string_set.contains(&binstr) {
            return binstr;
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::find_different_binary_string;
    #[test]
    fn run_tc1() {
        let nums = vec![String::from("10"), String::from("11")];
        assert_eq!(find_different_binary_string(nums), String::from("00"));
    }
    #[test]
    fn run_tc2() {
        let nums = vec![
            String::from("111"),
            String::from("011"),
            String::from("001"),
        ];
        assert_eq!(find_different_binary_string(nums), String::from("000"));
    }
}

fn main() {
    let nums = vec![
        String::from("111"),
        String::from("011"),
        String::from("001"),
    ];
    assert_eq!(find_different_binary_string(nums), String::from("000"));
}
