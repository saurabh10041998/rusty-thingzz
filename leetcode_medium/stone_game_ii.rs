use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Turn {
    Alice,
    Bob,
}
fn dfs(
    piles: &Vec<i32>,
    dp: &mut HashMap<(Turn, usize, i32), i32>,
    turn: Turn,
    idx: usize,
    m: i32,
) -> i32 {
    if idx == piles.len() {
        return 0;
    }
    match dp.get(&(turn, idx, m)) {
        Some(ans) => return *ans,
        None => {}
    };
    let mut res = if turn == Turn::Alice { 0 } else { i32::MAX };
    let mut total = 0;
    // Selection choices for each player
    for x in 1..=(2 * m) as usize {
        if idx + x > piles.len() {
            break;
        }
        total += piles[idx + x - 1];
        match turn {
            Turn::Alice => {
                // To play optimally, alice will try to maximize her score
                res = i32::max(
                    res,
                    total + dfs(piles, dp, Turn::Bob, idx + x, i32::max(m, x as i32)),
                );
            }
            Turn::Bob => {
                // To play optimally, bob will try to minimize alice score.
                res = i32::min(
                    res,
                    dfs(piles, dp, Turn::Alice, idx + x, i32::max(m, x as i32)),
                );
            }
        }
    }
    dp.entry((turn, idx, m)).or_insert(res);
    res
}

fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    dfs(&piles, &mut dp, Turn::Alice, 0, 1)
}

#[cfg(test)]
pub mod tests {
    use crate::stone_game_ii;
    #[test]
    fn run_tc1() {
        let piles = vec![2, 7, 9, 4, 4];
        assert_eq!(stone_game_ii(piles), 10);
    }
    #[test]
    fn run_tc2() {
        let piles = vec![1, 2, 3, 4, 5, 100];
        assert_eq!(stone_game_ii(piles), 104);
    }
}

fn main() {
    let piles = vec![2, 7, 9, 4, 4];
    assert_eq!(stone_game_ii(piles), 10);
}
