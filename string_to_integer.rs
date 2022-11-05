fn my_atoi(s: String) -> i32 {
    let mut i = 0;
    while i < s.len() && &s[i..i + 1] == " " {
        i += 1;
    }
    if i  == s.len() {
        return 0;
    }
    let mut sign = 1;
    if &s[i..i + 1] == "-" {
        sign = -1;
    }
    if &s[i..i + 1] == "-" || &s[i..i + 1] == "+" {
        i += 1;
    }
    let mut num = 0;
    let boundary = i32::MAX / 10;
    let partial:Vec<char> = s[i..].chars().collect();
    for c in partial {
        if c < 0x30 as char || c > 0x39 as char {
            break;
        }
        if num > boundary || (num == boundary && c > 0x37 as char) {
            if sign == -1 {
                return i32::MIN;
            }else {
                return i32::MAX;
            }
        }
        num = num * 10 + (c as u8 - 0x30) as  i32;
    }
    num * sign
}
fn main() {
    println!("Hello, world!");
    let testcase1 = String::from("   -42");
    println!("{}", my_atoi(testcase1));

    let testcase2 = String::from("4193 with words");
    println!("{}", my_atoi(testcase2));
}
