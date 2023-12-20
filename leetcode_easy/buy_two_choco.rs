//! The answer to this question is
//! find lowest and second lowest prices
//! and subtract it from money
//! If leftover is positive or zero, return leftover
//! else return the money
//! Many ways to implement this. Sort, heap, set etc.
//! I did simple implementation of keeping lowest and 
//! second lowest variables in one pass.
//!
//! One cool thing, that I came to find about Rust from this
//! problem
//!
//! Rust exepcts pattern (Instead of variable name) on the
//! left side of "=", which makes thingzz super easy
//! during scenario like the one below, declaring and
//! destructuring structs field at the same time. (See line 27-30)
//!
//! Super cool !!
//!
//!
struct Container {
    lowest: i32,
    second_lowest: i32,
}

fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let Container {
        lowest,
        second_lowest,
    } = prices.into_iter().fold(
        Container {
            lowest: i32::MAX - 1,
            second_lowest: i32::MAX,
        },
        |mut container, n| {
            if n <= container.lowest {
                container.second_lowest = container.lowest;
                container.lowest = n;
            } else if n < container.second_lowest {
                container.second_lowest = n;
            }
            container
        },
    );

    if money - (lowest + second_lowest) >= 0 {
        money - (lowest + second_lowest)
    } else {
        money
    }
}

#[cfg(test)]
pub mod tests {
    use crate::buy_choco;
    #[test]
    fn run_tc1() {
        let prices = vec![1, 2, 2];
        let money = 3;
        assert_eq!(buy_choco(prices, money), 0);
    }
    #[test]
    fn run_tc2() {
        let prices = vec![3, 2, 3];
        let money = 3;
        assert_eq!(buy_choco(prices, money), 3);
    }
}

fn main() {
    let prices = vec![3, 2, 3];
    let money = 3;
    assert_eq!(buy_choco(prices, money), 3);
}
