use std::mem::swap;
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        swap(&mut a, &mut b);
    }
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + str2.as_str() != str2.clone() + str1.as_str() {
        return String::from("");
    }
    let len = gcd(str1.len(), str2.len());

    String::from(&str1[..len])
}

#[cfg(test)]
pub mod tests {
    use crate::gcd_of_strings;
    #[test]
    fn run_tc1() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");
        assert_eq!(gcd_of_strings(str1, str2), String::from("ABC"));
    }
    #[test]
    fn run_tc2() {
        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        assert_eq!(gcd_of_strings(str1, str2), String::from("AB"));
    }
    #[test]
    fn run_tc3() {
        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        assert_eq!(gcd_of_strings(str1, str2), String::from(""));
    }
}

fn main() {
    let str1 = String::from("ABCABC");
    let str2 = String::from("ABC");
    assert_eq!(gcd_of_strings(str1, str2), String::from("ABC"));
}
