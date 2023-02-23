use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Project {
    profit: i32,
    capital: i32,
}

impl From<(i32, i32)> for Project {
    fn from(value: (i32, i32)) -> Self {
        Project {
            capital: value.0,
            profit: value.1,
        }
    }
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.profit == other.profit {
            return self.capital.partial_cmp(&other.capital);
        }
        self.profit.partial_cmp(&other.profit)
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.profit == other.profit {
            return self.capital.cmp(&other.capital);
        }
        self.profit.cmp(&other.profit)
    }
}

fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let n = profits.len();
    let mut projects = vec![];
    for (&c, &p) in capital.iter().zip(profits.iter()) {
        let p: Project = (c, p).into();
        projects.push(p);
    }
    // Sort by less capital
    projects.sort_by(|a, b| a.capital.cmp(&b.capital));
    // Order by max_profit 
    let mut pq = BinaryHeap::new();
    let (mut k, mut w, mut i) = (k, w, 0);
    while k > 0 {
        while i < n && projects[i].capital <= w {
            pq.push(projects[i].clone());
            i += 1;
        }
        match pq.pop() {
            Some(project) => {
                w += project.profit;
                k -= 1;
            }
            None => {
                break;
            }
        }
    }
    w
}

#[cfg(test)]
pub mod tests {
    use crate::find_maximized_capital;
    #[test]
    fn run_tc1() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        assert_eq!(find_maximized_capital(k, w, profits, capital), 4);
    }
    #[test]
    fn run_tc2() {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        assert_eq!(find_maximized_capital(k, w, profits, capital), 6);
    }
}

fn main() {
    let k = 3;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 2];
    assert_eq!(find_maximized_capital(k, w, profits, capital), 6);
}
