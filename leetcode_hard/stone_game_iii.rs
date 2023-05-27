use std::collections::HashMap;

fn dfs(values: &Vec<i32>, dp: &mut HashMap<usize, i32>, idx: usize) -> i32 {
    if idx == values.len() {
        return 0;
    }
    match dp.get(&idx) {
        Some(ans) => return *ans,
        None => {}
    };
    let mut res = i32::MIN;
    for j in idx..usize::min(values.len(), idx + 3) {
        res = i32::max(
            res,
            values[idx..j + 1].iter().sum::<i32>() - dfs(values, dp, j + 1),
        );
    }
    dp.entry(idx).or_insert(res);
    res
}

fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let mut dp = HashMap::new();
    let ans = dfs(&stone_value, &mut dp, 0);
    if ans < 0 {
        String::from("Bob")
    } else if ans == 0 {
        String::from("Tie")
    } else {
        String::from("Alice")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::stone_game_iii;
    #[test]
    fn run_tc1() {
        let stone_values = vec![1, 2, 3, 7];
        assert_eq!(stone_game_iii(stone_values), String::from("Bob"));
    }
    #[test]
    fn run_tc2() {
        let stone_values = vec![1, 2, 3, -9];
        assert_eq!(stone_game_iii(stone_values), String::from("Alice"));
    }

    #[test]
    fn run_tc3() {
        let stone_values = vec![1, 2, 3, 6];
        assert_eq!(stone_game_iii(stone_values), String::from("Tie"));
    }
}

fn main() {
    let stone_values = vec![1, 2, 3, 6];
    assert_eq!(stone_game_iii(stone_values), String::from("Tie"));
}
