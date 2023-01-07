fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    if {
        let gas_sum = gas.iter().sum::<i32>();
        let cost_sum = cost.iter().sum::<i32>();
        gas_sum < cost_sum
    } {
        return -1;
    }
    // answer exists
    let (mut total, mut start) = (0, 0);
    let n = gas.len();
    for i in 0..n {
        total += gas[i] - cost[i];
        if total < 0 {
            total = 0;
            start = i + 1;
        }
    }
    start as i32
}

#[cfg(test)]
pub mod tests {
    use crate::can_complete_circuit;
    #[test]
    fn run_tc1() {
        let (gas, cost) = (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
        assert_eq!(can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn run_tc2() {
        let (gas, cost) = (vec![2, 3, 4], vec![3, 4, 3]);
        assert_eq!(can_complete_circuit(gas, cost), -1);
    }
}

fn main() {
    let (gas, cost) = (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
    assert_eq!(can_complete_circuit(gas, cost), 3);
}
