fn number_of_beams(bank: Vec<String>) -> i32 {
    bank.into_iter()
        .fold((0i32, 0i32), |(mut ans, mut prev), row| {
            let curr = row
                .chars()
                .into_iter()
                .filter_map(|c| if c == '1' { Some(1) } else { None })
                .sum::<i32>();
            if curr != 0 {
                ans += prev * curr;
                prev = curr;
            }
            (ans, prev)
        })
        .0
}

#[cfg(test)]
pub mod tests {
    use crate::number_of_beams;
    #[test]
    fn run_tc1() {
        let bank = vec![
            String::from("011001"),
            String::from("000000"),
            String::from("010100"),
            String::from("001000"),
        ];
        assert_eq!(number_of_beams(bank), 8);
    }
    #[test]
    fn run_tc2() {
        let bank = vec![
            String::from("000"),
            String::from("111"),
            String::from("000"),
        ];
        assert_eq!(number_of_beams(bank), 0);
    }
}

fn main() {
    let bank = vec![
        String::from("011001"),
        String::from("000000"),
        String::from("010100"),
        String::from("001000"),
    ];
    assert_eq!(number_of_beams(bank), 8);
}
