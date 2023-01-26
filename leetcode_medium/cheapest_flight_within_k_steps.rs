use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct FlightDescriptor {
    from: usize,
    to: usize,
    price: i32,
}

impl From<&Vec<i32>> for FlightDescriptor {
    fn from(buffer: &Vec<i32>) -> Self {
        FlightDescriptor {
            from: buffer[0] as usize,
            to: buffer[1] as usize,
            price: buffer[2],
        }
    }
}

fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut adj_lst = vec![HashSet::new(); n as usize];
    for flight in flights.iter() {
        let f: FlightDescriptor = flight.into();
        adj_lst[f.from].insert((f.to, f.price));
    }
    let mut min_dist = vec![i32::MAX; n as usize];
    let mut q = VecDeque::new();
    q.push_back((src as usize, 0));
    min_dist[src as usize] = 0;
    let mut stops = 0;
    while !q.is_empty() && stops <= k {
        let mut size = q.len();
        while size > 0 {
            let (curr, cost) = q.pop_front().unwrap();
            for &(neighbor, price) in adj_lst[curr].iter() {
                if price + cost < min_dist[neighbor] {
                    min_dist[neighbor] = cost + price;
                    q.push_back((neighbor, min_dist[neighbor]));
                }
            }
            size -= 1;
        }
        stops += 1;
    }
    match min_dist[dst as usize] {
        i32::MAX => -1,
        v => v,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::find_cheapest_price;
    #[test]
    fn run_tc1() {
        let n = 4;
        let src = 0;
        let dst = 3;
        let k = 1;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        assert_eq!(find_cheapest_price(n, flights, src, dst, k), 700);
    }

    #[test]
    fn run_tc2() {
        let n = 3;
        let src = 0;
        let dst = 2;
        let k = 1;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(n, flights, src, dst, k), 200);
    }
    #[test]
    fn run_tc3() {
        let n = 3;
        let src = 0;
        let dst = 2;
        let k = 0;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(n, flights, src, dst, k), 500);
    }
}

fn main() {
    let n = 4;
    let src = 0;
    let dst = 3;
    let k = 1;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 700);
}
