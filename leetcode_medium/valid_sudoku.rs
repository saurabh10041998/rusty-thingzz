use std::collections::HashMap;
use std::collections::HashSet;

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut grids: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    for r in 0..9 as usize {
        for c in 0..9 as usize {
            if board[r][c] == '.' {
                continue;
            } else {
                // checking rows
                let ok1 = match rows.get(&r) {
                    Some(v) => v.contains(&board[r][c]),
                    None => false,
                };
                // checking columns
                let ok2 = match cols.get(&c) {
                    Some(v) => v.contains(&board[r][c]),
                    None => false,
                };
                // check in 3 * 3 grids
                let ok3 = match grids.get(&(r.checked_div(3).unwrap(), c.checked_div(3).unwrap())) {
                    Some(v) => v.contains(&board[r][c]),
                    None => false,
                };
                if ok1 || ok2 || ok3 {
                    return false;
                } else {
                    rows.entry(r)
                        .and_modify(|val| {
                            val.insert(board[r][c]);
                        })
                        .or_insert_with(|| HashSet::from([board[r][c]]));

                    cols.entry(c)
                        .and_modify(|val| {
                            val.insert(board[r][c]);
                        })
                        .or_insert_with(|| HashSet::from([board[r][c]]));

                    grids
                        .entry((r.checked_div(3).unwrap(), c.checked_div(3).unwrap()))
                        .and_modify(|val| {
                            val.insert(board[r][c]);
                        })
                        .or_insert_with(|| HashSet::from([board[r][c]]));
                }
            }
        }
    }
    // just to debug print 
    // to see following debug output
    // use cargo test -- --nocapture
    dbg!(rows);
    dbg!(cols);
    dbg!(grids);
    true
}

#[cfg(test)]
pub mod tests {
    use crate::is_valid_sudoku;
    #[test]
    fn run_tc1() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }
    #[test]
    fn run_tc2() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }
}

fn main() {
    println!("Hello, world!");
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(is_valid_sudoku(board), true);
}
