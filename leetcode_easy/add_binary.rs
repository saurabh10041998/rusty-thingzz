fn add_binary(a: String, b: String) -> String {
    if b.len() > a.len() {
        return add_binary(b, a);
    }
    let diff = a.len() - b.len();
    let mut padding = String::new();
    for _ in 0..diff {
        padding = padding + "0";
    }
    let new_b = padding + &b[..];
    let mut res = String::new();
    let mut carry = "0";
    for (x, y) in a.chars().rev().zip(new_b.chars().rev()) {
        if x == '1' && y == '1' {
            if carry == "1" {
                res = res + "1";
                carry = "1";
            }else {
                res = res + "0";
                carry = "1";
            }
        } else if x == '0' && y == '0' {
            if carry == "1" {
                res = res + "1";
                carry = "0";
            }else {
                res = res + "0";
                carry = "0";
            }
        }else {   // 0  + 1 or 1  + 0
            if carry == "1" {
                res = res + "0";
                carry = "1";
            }else {
                res = res + "1";
                carry = "0";
            }
        }
    }
    if carry == "1" {
        res = res + carry;
    }
    let res = res.chars().rev().collect::<String>();
    let mut index = 0;
    while index + 1 < res.len() &&  &res[index..index+1] == "0" {
        index += 1;
    }
    String::from(&res[index..])
}

#[cfg(test)] 
pub mod tests {
    use super::*;
    #[test]
    fn check_test1() {
        let a = String::from("11");
        let b = String::from("1");
        assert_eq!(String::from("100"), add_binary(a, b));
    }
    #[test]
    fn check_test2() {
        let a = String::from("1010");
        let b = String::from("1011");
        assert_eq!(String::from("10101"), add_binary(a, b));
    }
}


fn main() {
    println!("Hello, world!");
    let a = String::from("11");
    let b = String::from("1");
    assert_eq!(String::from("100"), add_binary(a, b));
}
