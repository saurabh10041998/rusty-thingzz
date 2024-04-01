fn is_alpha(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    }
}

fn length_of_last_word(s: String) -> i32 {
    let (mut count, mut counting) = (0, false);
    let s = s.chars().rev().collect::<Vec<char>>();
    for x in s {
        if is_alpha(x) {
            counting = true;
            count += 1;
        } else {
            if counting {
                break;
            }
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::length_of_last_word;
    #[test]
    fn run_tc1() {
        let s = String::from("Hello World");
        assert_eq!(length_of_last_word(s), 5);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("   fly me   to   the moon  ");
        assert_eq!(length_of_last_word(s), 4);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("luffy is still joyboy");
        assert_eq!(length_of_last_word(s), 6);
    }
}

fn main() {
    let s = String::from("   fly me   to   the moon  ");
    assert_eq!(length_of_last_word(s), 4);
}
