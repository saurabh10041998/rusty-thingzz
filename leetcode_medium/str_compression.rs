fn compress(chars: &mut Vec<char>) -> i32 {
    let n = chars.len();
    let (mut i, mut j) = (0usize, 0usize);
    let mut cnt;
    while i < n {
        cnt = 1;
        while i < n - 1 && chars[i] == chars[i + 1] {
            i += 1;
            cnt += 1;
        }
        chars[j] = chars[i];
        j += 1;
        i += 1;
        if cnt != 1 {
            let s = format!("{}", cnt);
            for c in s.chars() {
                chars[j] = c;
                j += 1;
            }
        }
    }
    j as i32
}

#[cfg(test)]
pub mod tests {
    use crate::compress;
    #[test]
    fn run_tc1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let len = 6;
        assert_eq!(compress(&mut chars), len as i32);
        let s = chars.iter().take(len).collect::<String>();
        assert_eq!(s, String::from("a2b2c3"));
    }
    #[test]
    fn run_tc2() {
        let mut chars = vec!['a'];
        let len = 1;
        assert_eq!(compress(&mut chars), len as i32);
        let s = chars.iter().take(len).collect::<String>();
        assert_eq!(s, String::from("a"));
    }

    #[test]
    fn run_tc3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let len = 4;
        assert_eq!(compress(&mut chars), len as i32);
        let s = chars.iter().take(len).collect::<String>();
        assert_eq!(s, String::from("ab12"));
    }
}

fn main() {
    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    let len = 4;
    assert_eq!(compress(&mut chars), len as i32);
    let s = chars.iter().take(len).collect::<String>();
    assert_eq!(s, String::from("ab12"));
}
