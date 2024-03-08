use std::collections::HashMap;

fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let (max_freq, hs) =
        nums.into_iter()
            .fold((0, HashMap::new()), |(mut max_freq, mut map), ele| {
                let delta;
                match map.remove(&ele) {
                    Some(freq) => {
                        delta = freq + 1;
                    }
                    None => {
                        delta = 1;
                    }
                }
                max_freq = i32::max(delta, max_freq);
                map.entry(ele).or_insert(delta);
                (max_freq, map)
            });
    let mut ans = 0;
    for (_, v) in hs.into_iter() {
        if v == max_freq {
            ans += v;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::max_frequency_elements;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 2, 3, 1, 4];
        assert_eq!(max_frequency_elements(nums), 4);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(max_frequency_elements(nums), 5);
    }
}

fn main() {
    let nums = vec![1, 2, 2, 3, 1, 4];
    assert_eq!(max_frequency_elements(nums), 4);
}
