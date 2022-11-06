fn legnth_of_last_word(s: String) -> i32 {
    let mut flag = false;
    let mut count = 0;
    let chrlst: Vec<char> = s.chars().rev().collect();
    for c in chrlst {
        if (c >= 0x41 as char && c <= 0x5a as char) || (c >= 0x61 as char && c <= 0x7a as char) {
            flag = true;
            count += 1;
        } else {
            if flag {
                break;
            }
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let s: String = String::from("Hello World");
        assert_eq!(5, legnth_of_last_word(s));
    }

    #[test]
    fn run_tc2() {
        let s: String = String::from("   fly me   to   the moon  ");
        assert_eq!(4, legnth_of_last_word(s));
    }

    #[test]
    fn run_tc3() {
        let s: String = String::from("luffy is still joyboy");
        assert_eq!(6, legnth_of_last_word(s));
    }
}
fn main() {
    println!("Hello, world!");
    // *** If you run the main code
    let s: String = String::from("luffy is still joyboy");
    assert_eq!(6, legnth_of_last_word(s));
}
