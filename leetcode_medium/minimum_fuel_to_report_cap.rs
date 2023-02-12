use std::collections::HashSet;

fn ceil(a: i64, b: i64) -> i64 {
    a / b + (a % b != 0) as i64
}

fn dfs(
    adj_lst: &Vec<HashSet<usize>>,
    src: usize,
    parent: Option<usize>,
    fuel: &mut i64,
    seats: &i64,
) -> i64 {
    let mut people = 1;
    for &v in adj_lst[src].iter() {
        people += match parent {
            None => dfs(adj_lst, v, Some(src), fuel, seats),
            Some(u) if u == v => 0,
            Some(_u) => dfs(adj_lst, v, Some(src), fuel, seats),
        }
    }
    if src != 0 {
        *fuel += ceil(people, *seats);
    };
    people
}

fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let n = roads.len();
    let mut adj_lst = vec![HashSet::new(); n + 1];
    for e in roads.iter() {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj_lst[u].insert(v);
        adj_lst[v].insert(u);
    }
    let seats = seats as i64;
    let mut fuel = 0;
    dfs(&adj_lst, 0, None, &mut fuel, &seats);
    fuel
}

#[cfg(test)]
pub mod tests {
    use crate::minimum_fuel_cost;
    #[test]
    fn run_tc1() {
        let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let seats = 5;
        assert_eq!(minimum_fuel_cost(roads, seats), 3);
    }
    #[test]
    fn run_tc2() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let seats = 2;
        assert_eq!(minimum_fuel_cost(roads, seats), 7);
    }
    #[test]
    fn run_tc3() {
        let roads = vec![];
        let seats = 1;
        assert_eq!(minimum_fuel_cost(roads, seats), 0);
    }
}

fn main() {
    let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let seats = 5;
    assert_eq!(minimum_fuel_cost(roads, seats), 3);
}
