use std::collections::HashSet;

pub struct Solver {
    board: Vec<Vec<char>>,
    word: String,
    path: HashSet<(usize, usize)>,
}

impl Solver {
    fn new(board: Vec<Vec<char>>, word: String) -> Self {
        Solver {
            board,
            word,
            path: HashSet::new(),
        }
    }
    fn solve(&mut self) -> bool {
        let (n, m) = (self.board.len(), self.board[0].len());
        for r in 0..n {
            for c in 0..m {
                if self.dfs(r, c, 0, n, m) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(&mut self, r: usize, c: usize, i: usize, n: usize, m: usize) -> bool {
        if i == self.word.len() {
            return true;
        }
        if r >= n
            || c >= m
            || self.board[r][c] != *&self.word[i..i + 1].chars().nth(0).unwrap()
            || self.path.contains(&(r, c))
        {
            return false;
        }
        self.path.insert((r, c));

        let res1 = match r.checked_sub(1) {
            Some(_) => self.dfs(r - 1, c, i + 1, n, m),
            None => false,
        };
        let res2 = match r.checked_add(1) {
            Some(_) => self.dfs(r + 1, c, i + 1, n, m),
            None => false,
        };
        let res3 = match c.checked_add(1) {
            Some(_) => self.dfs(r, c + 1, i + 1, n, m),
            None => false,
        };
        let res4 = match c.checked_sub(1) {
            Some(_) => self.dfs(r, c - 1, i + 1, n, m),
            None => false,
        };

        let res = res1 || res2 || res3 || res4;
        self.path.remove(&(r, c));
        res
    }
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut s = Solver::new(board, word);
    s.solve()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word: String = String::from("ABCCED");
        assert_eq!(exist(board, word), true);
    }

    #[test]
    fn run_tc2() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word: String = String::from("SEE");
        assert_eq!(exist(board, word), true);
    }
    #[test]
    fn run_tc3() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word: String = String::from("ABCB");
        assert_eq!(exist(board, word), false);
    }
}

fn main() {
    println!("Hello, world!");
    let board: Vec<Vec<char>> = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word: String = String::from("ABCCED");
    assert_eq!(exist(board, word), true);
}
