use std::collections::BTreeSet;
use std::ops::Bound::*;

pub struct Sieve {
    bitset: Vec<bool>,
    primes: Vec<i64>,
    n: usize,
}

// 32000 ~= sqrt(10 ** 9)

impl Sieve {
    fn new(n: usize) -> Self {
        Sieve {
            bitset: vec![true; n + 1],
            primes: Vec::new(),
            n,
        }
    }

    fn cal(&mut self) {
        let mut i = 2;
        while i * i < self.n + 1 {
            if self.bitset[i] {
                let mut j = i * i;
                while j < self.n + 1 {
                    self.bitset[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        for i in 0..self.n + 1 {
            if i > 1 && self.bitset[i] {
                self.primes.push(i as i64);
            }
        }
    }
}

pub struct Solver {
    sieve: Sieve,
}

impl Solver {
    fn new(n: usize) -> Self {
        Solver {
            sieve: Sieve::new(n),
        }
    }

    fn generate_sieve(&mut self) {
        self.sieve.cal();
    }
}

fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let mut solver = Solver::new(i32::pow(10, 6) as usize);
    solver.generate_sieve();
    let bt = solver.sieve.primes.drain(..).collect::<BTreeSet<i64>>();
    let nums = bt.range((Included(left as i64), Included(right as i64)));

    let primes = nums.collect::<Vec<&i64>>();

    if primes.len() == 0 || primes.len() == 1 {
        return vec![-1, -1];
    }
    let mut diff = i64::MAX;
    let mut pair = (-1, -1);
    for i in 0..primes.len() - 1 {
        if *primes[i + 1] - *primes[i] < diff {
            pair.0 = *primes[i];
            pair.1 = *primes[i + 1];
            diff = *primes[i + 1] - *primes[i];
        }
    }
    let mut ans = vec![];
    ans.push(pair.0 as i32);
    ans.push(pair.1 as i32);
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let left = 10;
        let right = 19;
        assert_eq!(closest_primes(left, right), vec![11, 13]);
    }
    #[test]
    fn run_tc2() {
        let left = 4;
        let right = 6;
        assert_eq!(closest_primes(left, right), vec![-1, -1]);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    let left = 10;
    let right = 19;
    assert_eq!(closest_primes(left, right), vec![11, 13]);
}
