fn _count_and_say(s: String, remaining_steps: i32) -> String {
    if remaining_steps == 0 {
        return s;
    }
    let arr = s.chars().collect::<Vec<char>>();
    let n = arr.len();
    let mut curr = arr[0];
    let mut cnt = 1;
    let mut ans = String::new();
    for i in 1..n {
        if curr == arr[i] {
            cnt += 1;
        } else {
            ans.push_str(&format!("{}", cnt));
            ans.push_str(&format!("{}", curr));
            curr = arr[i];
            cnt = 1;
        }
    }
    ans.push_str(&format!("{}", cnt));
    ans.push_str(&format!("{}", curr));
    _count_and_say(ans, remaining_steps - 1)
}

fn count_and_say(n: i32) -> String {
    _count_and_say(String::from("1"), n - 1)
}

#[cfg(test)]
mod tests {
    use crate::count_and_say;
    #[test]
    fn run_tc1() {
        let n = 4;
        assert_eq!(count_and_say(n), String::from("1211"));
    }
    #[test]
    fn run_tc2() {
        let n = 1;
        assert_eq!(count_and_say(n), String::from("1"));
    }
}

fn main() {
    let n = 4;
    assert_eq!(count_and_say(n), String::from("1211"));
}
