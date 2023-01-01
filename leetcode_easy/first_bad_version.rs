use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Solution;

// Only for testing purpose
// !! Guessed implementation..
fn is_bad_version(version: i32) -> bool {
    let fixed_random = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (version as u64 + fixed_random) % 2 == 0
}

impl Solution {
    fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        while low < high {
            let mid = low + (high - low) / 2;
            let resp = is_bad_version(mid);
            println!("{} {}", mid, resp);
            if resp {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

fn main() {
    let sol = Solution {};
    println!("{}", sol.first_bad_version(1));
}
