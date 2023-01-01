use std::collections::HashSet;
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
fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
    let mut solver = Solver::new(1000);
    solver.generate_sieve();
    let mut hs = HashSet::new();
    
    for n in nums { 
        let mut key = n as i64;
        for &c in solver.sieve.primes.iter() {
            if key  != 0 && key % c == 0 {
                hs.insert(c);
                while key != 0 && key % c == 0 {
                    key /= c;
                }
            }
        }
    }
    hs.len() as i32
} 


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![2,4,3,7,10,6];
        assert_eq!(distinct_prime_factors(nums), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2,4,8,16];
        assert_eq!(distinct_prime_factors(nums), 1);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![5, 3];
        assert_eq!(distinct_prime_factors(nums), 2);
    }
}
fn main() {
    println!("Hello, world!");
}
