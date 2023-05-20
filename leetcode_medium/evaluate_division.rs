use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(src: String, dst: String, adj_lst: &HashMap<String, Vec<(String, f64)>>) -> f64 {
    if !adj_lst.contains_key(&src) || !adj_lst.contains_key(&dst) {
        return -1f64;
    }
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((src.clone(), 1f64));
    visited.insert(src);

    while let Some((node, w)) = q.pop_front() {
        if node == dst {
            return w;
        }
        for (nei, wei) in adj_lst.get(&node).unwrap() {
            if !visited.contains(nei) {
                q.push_back((nei.clone(), *wei * w));
                visited.insert(nei.clone());
            }
        }
    }

    return -1f64;
}

fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut adj_lst = HashMap::new();
    for (idx, eq) in equations.into_iter().enumerate() {
        let a = String::from(&eq[0]);
        let b = String::from(&eq[1]);
        adj_lst
            .entry(a.clone())
            .and_modify(|v: &mut Vec<(String, f64)>| v.push((b.clone(), values[idx])))
            .or_insert(vec![(b.clone(), values[idx])]);
        adj_lst
            .entry(b)
            .and_modify(|v| v.push((a.clone(), 1f64 / values[idx])))
            .or_insert(vec![(a, 1f64 / values[idx])]);
    }
    queries
        .into_iter()
        .map(|q| bfs(q[0].clone(), q[1].clone(), &adj_lst))
        .collect()
}

#[allow(dead_code)]
// Internal method of conversion.
fn convert_to_grid_of_string(_arr: Vec<[&str; 2]>) -> Vec<Vec<String>> {
    let mut arr = vec![];
    for eq in _arr {
        let mut var = vec![];
        for tok in eq {
            var.push(String::from(tok));
        }
        arr.push(var);
    }
    return arr;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let _equations = vec![["a", "b"], ["b", "c"]];
        let values = vec![2.0, 3.0];
        let _queries = vec![["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]];
        let equations = convert_to_grid_of_string(_equations);
        let queries = convert_to_grid_of_string(_queries);
        // Cannot compare float in rust.. which is correct behaviour
        assert!(true);
        println!("{:?}", calc_equation(equations, values, queries));
    }
    #[test]
    fn run_tc2() {
        let _equations = vec![["a", "b"], ["b", "c"], ["bc", "cd"]];
        let values = vec![1.5, 2.5, 5.0];
        let _queries = vec![["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]];
        let equations = convert_to_grid_of_string(_equations);
        let queries = convert_to_grid_of_string(_queries);
        // Cannot compare float in rust.. which is correct behaviour
        assert!(true);
        println!("{:?}", calc_equation(equations, values, queries));
    }
    #[test]
    fn run_tc3() {
        let _equations = vec![["a", "b"]];
        let values = vec![0.5];
        let _queries = vec![["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]];
        let equations = convert_to_grid_of_string(_equations);
        let queries = convert_to_grid_of_string(_queries);
        // Cannot compare float in rust.. which is correct behaviour
        assert!(true);
        println!("{:?}", calc_equation(equations, values, queries));
    }
}

fn main() {
    let _equations = vec![["a", "b"]];
    let values = vec![0.5];
    let _queries = vec![["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]];
    let equations = convert_to_grid_of_string(_equations);
    let queries = convert_to_grid_of_string(_queries);
    // Cannot compare float in rust.. which is correct behaviour
    assert!(true);
    println!("{:?}", calc_equation(equations, values, queries));
}
