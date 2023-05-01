fn average(salary: Vec<i32>) -> f64 {
    let min = *salary.iter().min().unwrap();
    let max = *salary.iter().max().unwrap();
    let sum = salary.iter().sum::<i32>();

    (sum - min - max) as f64 / (salary.len() as f64 - 2.0)
}

#[cfg(test)]
pub mod tests {
    use crate::average;
    #[test]
    fn run_tc1() {
        let salary = vec![4000, 3000, 1000, 2000];
        assert_eq!(average(salary) as i64, 2500);
    }
    #[test]
    fn run_tc2() {
        let salary = vec![3000, 1000, 2000];
        assert_eq!(average(salary) as i64, 2000);
    }
}

fn main() {
    let salary = vec![3000, 1000, 2000];
    assert_eq!(average(salary) as i64, 2000);
}
