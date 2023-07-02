struct Request {
    from: usize,
    to: usize,
}
impl From<&Vec<i32>> for Request {
    fn from(value: &Vec<i32>) -> Self {
        assert_eq!(value.len(), 2);
        Request {
            from: value[0] as usize,
            to: value[1] as usize,
        }
    }
}
struct Backtracker {
    buildings: Vec<i32>,
    best: i32,
}

impl Backtracker {
    fn new(n: usize) -> Self {
        Backtracker {
            buildings: vec![0; n],
            best: i32::MIN,
        }
    }

    fn solve(&mut self, requests: &[Vec<i32>], idx: usize, cnt: i32) {
        if idx == requests.len() {
            if self.buildings.iter().all(|&x| x == 0) {
                self.best = i32::max(self.best, cnt);
            }
            return;
        }
        // dont process the request
        self.solve(requests, idx + 1, cnt);

        //process the request
        let r: Request = (&requests[idx]).into();
        self.buildings[r.from] -= 1;
        self.buildings[r.to] += 1;
        self.solve(requests, idx + 1, cnt + 1);
        self.buildings[r.to] -= 1;
        self.buildings[r.from] += 1;
    }

    fn get_best(&self) -> i32 {
        self.best
    }
}

fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut bk = Backtracker::new(n as usize);
    bk.solve(&requests, 0, 0);
    bk.get_best()
}

#[cfg(test)]
pub mod tests {
    use crate::maximum_requests;
    #[test]
    fn run_tc1() {
        let n = 5;
        let requests = vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ];
        assert_eq!(maximum_requests(n, requests), 5);
    }
    #[test]
    fn run_tc2() {
        let n = 3;
        let requests = vec![vec![0, 0], vec![1, 2], vec![2, 1]];
        assert_eq!(maximum_requests(n, requests), 3);
    }
    #[test]
    fn run_tc3() {
        let n = 4;
        let requests = vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]];
        assert_eq!(maximum_requests(n, requests), 4);
    }
}
fn main() {
    let n = 5;
    let requests = vec![
        vec![0, 1],
        vec![1, 0],
        vec![0, 1],
        vec![1, 2],
        vec![2, 0],
        vec![3, 4],
    ];
    assert_eq!(maximum_requests(n, requests), 5);
}
