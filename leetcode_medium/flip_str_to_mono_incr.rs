fn min_flips_mono_incr(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut one = 0;
    let mut dp = vec![0; n + 1];
    dp[0] = 0; // Base case as empty string is always monotonically increasing
    for (i, &c) in s.iter().enumerate() {
        let i = i + 1; // Array is operating at 1 based idx
        match c {
            '0' => {
                // We got two choice
                // Either flip zero : This will contribute one to prev calculated ans
                // i.e. dp[i - 1] + 1      ... choice 1
                // Dont flip 0, flip all ones occured so far
                // i.e one                 ... choice 2
                // take the min of choice 1 and choice 2
                dp[i] = i32::min(dp[i - 1] + 1, one);
            }
            '1' => {
                // Then 1 will not disturb the ordering..
                // Just copy the previous calculated ans
                dp[i] = dp[i - 1];
                // Maintain count.. as in case 0 we have to decide
                // If flipping total #0 is benificial or total #1 is benificial or not
                one += 1;
            }
            _ => panic!("[!!] Its binary string, what are you {} doing here ??", c),
        }
    }
    // Return ans for entire substring
    dp[n]
}

#[cfg(test)]
pub mod tests {
    use crate::min_flips_mono_incr;
    #[test]
    fn run_tc1() {
        let s = String::from("00110");
        assert_eq!(min_flips_mono_incr(s), 1);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("010110");
        assert_eq!(min_flips_mono_incr(s), 2);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("00011000");
        assert_eq!(min_flips_mono_incr(s), 2);
    }
}

fn main() {
    let s = String::from("00110");
    assert_eq!(min_flips_mono_incr(s), 1);
}
