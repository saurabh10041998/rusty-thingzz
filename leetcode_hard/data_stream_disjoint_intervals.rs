use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
pub struct SummaryRanges {
    merger: BTreeSet<i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            merger: BTreeSet::new(),
        }
    }
    fn add_num(&mut self, value: i32) {
        self.merger.insert(value);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut intervals = vec![];
        let mut start = *self.merger.iter().nth(0).unwrap();
        let mut end = *self.merger.iter().nth(0).unwrap();
        for &val in self.merger.iter().skip(1) {
            if val - end == 1 {
                end = val;
            } else {
                intervals.push(vec![start, end]);
                start = val;
                end = val;
            }
        }
        intervals.push(vec![start, end]);
        intervals
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut summary_ranges = SummaryRanges::new();
        summary_ranges.add_num(1);
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]);
        summary_ranges.add_num(3);
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        summary_ranges.add_num(7);
        assert_eq!(
            summary_ranges.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        );
        summary_ranges.add_num(2);
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
        summary_ranges.add_num(6);
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}

fn main() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]);
    summary_ranges.add_num(3);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
    summary_ranges.add_num(7);
    assert_eq!(
        summary_ranges.get_intervals(),
        vec![vec![1, 1], vec![3, 3], vec![7, 7]]
    );
    summary_ranges.add_num(2);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
    summary_ranges.add_num(6);
    assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
}
