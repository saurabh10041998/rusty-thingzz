fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut res = String::new();
    let incr = (2 * (num_rows - 1)) as usize;
    for r in 0..num_rows as usize {
        for j in (r..s.len()).step_by(incr) {
            res.push_str(&s[j..j + 1]);
            if r > 0 && r < (num_rows - 1) as usize && j + (incr) - 2 * r < s.len() {
                let pos = j + incr - 2 * r;
                res.push_str(&s[pos..pos + 1])
            }
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::convert;
    #[test]
    fn run_tc1() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), String::from("PAHNAPLSIIGYIR"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        assert_eq!(convert(s, num_rows), String::from("PINALSIGYAHRPI"));
    }
    #[test]
    fn run_tc3() {
        let s = String::from("A");
        let num_rows = 1;
        assert_eq!(convert(s, num_rows), String::from("A"));
    }
}
fn main() {
    let s = String::from("PAYPALISHIRING");
    let num_rows = 4;
    assert_eq!(convert(s, num_rows), String::from("PINALSIGYAHRPI"));
}
