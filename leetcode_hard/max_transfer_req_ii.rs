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

fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut best = 0;
    let m = requests.len();
    for mask in 0..(1 << m) {
        let mut buildings = vec![0; n as usize];
        let mut cnt = 0;
        for (idx, req) in requests.iter().enumerate() {
            if (1 << idx) & mask > 0 {
                let r: Request = req.into();
                buildings[r.from] -= 1;
                buildings[r.to] += 1;
                cnt += 1;
            }
        }
        if buildings.into_iter().all(|x| x == 0) {
            best = i32::max(best, cnt);
        }
    }
    best
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
