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

fn smallest_value(n: i32) -> i32 {
    let mut n = n;
    let mut solver = Solver::new(n as usize);
    solver.generate_sieve();
    let mut prev = -1;
    while prev != n && !solver.sieve.bitset[n as usize] {
        let mut sum = 0;
        let mut j = 0;
        prev = n;
        while n != 1 {
            while n % solver.sieve.primes[j] as i32 == 0 {
                sum += solver.sieve.primes[j] as i32;
                n /= solver.sieve.primes[j] as i32;
            }
            j += 1;
        }
        n = sum;
    }
    n
}

#[cfg(test)]
pub mod tests {
    use crate::smallest_value;

    #[test]
    fn run_tc1() {
        let n = 15;
        assert_eq!(smallest_value(n), 5);
    }

    #[test]
    fn run_tc2() {
        let n = 4;
        assert_eq!(smallest_value(n), 4);
    }

    #[test]
    fn run_tc3() {
        let n = 3;
        assert_eq!(smallest_value(n), 3);
    }
}

fn main() {}
