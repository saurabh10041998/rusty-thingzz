fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left = 0;
    let mut right = letters.len() - 1;

    while left <= right {
        let offset = match right.checked_sub(left) {
            Some(val) => val,
            None => break,
        };
        let mid = left + offset / 2;
        match letters[mid].cmp(&target) {
            std::cmp::Ordering::Greater => {
                right = match mid.checked_sub(1) {
                    Some(new_right) => new_right,
                    None => break,
                };
            }
            _ => {
                left = mid + 1;
            }
        }
    }
    if left == letters.len() {
        letters[0]
    } else {
        letters[left]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::next_greatest_letter;
    #[test]
    fn run_tc1() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        assert_eq!(next_greatest_letter(letters, target), 'c');
    }
    #[test]
    fn run_tc2() {
        let letters = vec!['x', 'x', 'y', 'y'];
        let target = 'z';
        assert_eq!(next_greatest_letter(letters, target), 'x');
    }
    #[test]
    fn run_tc3() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        assert_eq!(next_greatest_letter(letters, target), 'f');
    }
}

fn main() {
    let letters = vec!['c', 'f', 'j'];
    let target = 'c';
    assert_eq!(next_greatest_letter(letters, target), 'f');
}
